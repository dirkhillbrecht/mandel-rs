# Manifesto 2: Refactoring of the storage and computation engine

_2025-06-13, Dirk Hillbrecht_

## Problem and requirements

Finishing the first stage, mandel-rs can create fractal images. It uses a clear distinction between _computation_ and _visualization_ using a storage layer in between.

However, this storage layer does not fulfill the needs of parallel access, especially when it comes to Rusts quite profound language architecture which prevents race conditions in such scenarios. The currently implemented `DataStorage` struct with an internal `DataPlane` struct is strictly single-access. It does not allow concurrent access to the data. This concurrent access will happen, however, on two occasions in the future:

1. Visualization needs data from the storage even during computation as it presents the ongoing progress during computation.
2. Computation itself will be split into parallel processes (or threads) so that parts of the image are computed simultaneously.

Parallelism in computation will additionally be dynamic, i.e. data will _not_ be simply devided into subregions which are computed independently. All computation processes have to be able to access data in the whole image any time.

## Inadequate solution strategies

The main point was Rusts strict memory handling regime and the requirement that all mutable memory can only be accessed at _one_ place at the same time. While accessing from different program parts can be handled with the `Arc` construct, this only helps for lifetime management of _immutable_ storage.

While tinkering with the problem, several solution strategies proved to be inadequate:

### Cloning DataStorage for visualization

The primary `DataStorage` could be kept more or less confined in the computation realm. Then, for visualization, a immutable clone containing a "snapshot" of the data is generated and handed over. However, this has several problems:

* Generating the clone becomes quite work-intensive if we go to real high resolution images (e.g. 8K). This will be up to 0.5 GB of data for each clone.
* While the clone is generated, computation cannot access the storage.
* The problem of parallel computation is not tackled at all.

### Moving `Arc` into the Storage

The most basic approach was to have `Arc<DataStorage>` where `DataStorage` uses internally a `Vec<DataPoint>`. The `Arc` could be moved downward into the stack so that it uses a `Vec<Arc<DataPoint>>`. This makes accessing multiple data points from different computation processes or from the visualization part possible without cloning all the data every time. Thinking through this more thoroughly, the conclusion was that at some point, cloning the data became inevitable, too.

This all lead to nothing.

## Solution strategy

We need to refine the whole data storage layer of the program. I have the following idea:

* Computation gets its own data handling layer which is designed for highly-concurrent access of different computation threads.
* Visualization stays with the current `DataStorage`/`DataPlane` design.
* The data for visualization can be initialized from the data within the storage in an exclusive-access operation on the data.
* Ongoing computation process is passed from computation to visualization by "change messages" in an event-style manner.

Let's dig deeper into this approach:

### Data handling in the computation layer

The main problem lies in the single-access `DataPlane` used within `DataStorage`. For parallel access by multiple computation threads, a more sophisticated data storage is needed. We handle this under the name `ComputationStage`. Its main design features are:

* Fixed 2-dimensional size for points.
* Capacity to store information for each point in the plane. This resembles `DataPlane` in the current setup. Perhaps we can use the current `DataPoint` structs for these.
* Concurrent access of many threads on all the points.
* Capability to change data of a point by each of the threads where the area of exclusive access for the writer is as small as possible, ideally this very point only.

The `ComputationStage` is stored in something very similar to `DataStorage`, let's call it `ComputationStorage`. It contains the same information as `DataStorage`, only that it does not refer to `DataPlane`, but to `ComputationStage`.

It makes sense to collect all the computation parameters (coordinates, width, height, max. iterations) in a new `ImageParameters` struct which can be shared between `ComputationStorage` and `DataStorage`.

### Generation of data for the visualization layer

To initialize visualization data, a function `ComputationStorage::initVisualizationStorage` creates the "classic" `DataStorage`/`DataPlane` structs out of the computation storage. It creates a completely independent data structure, i.e. it clones that `DataPoint` data.

This operation may block access to the whole `ComputationStorage` struct as it should be executed only seldomly. If an implementation with some non-exclusive access pattern can be implemented without problems, that would be even better. But actually, this operation will only be executed once in a while because ongoing information transfer from computation to visualization is done via "change messages".

### Change messages from computation to visualization

Everytime when the computation algorithm computes some points and stores them in the `ComputationStage`, a data structure is generated which describes the change. There are two such change operations which seem sensible regarding the problem realm:

1. Change one point at a certain x/y coordinate
2. Change a sequence of consecutive points between two x values and the same y value _with the same data_, i.e. set the same data to some points on a horizontal line

This seems a bit random, but the reasoning behind this set of operations is

* _All_ changes can be described with a series of type-1-operations.
* The boundary-trace-based optimized computation algorithm which I want to implement somewhen in the future will create lots of horizontal-line operations.

These messages will be sent from the computation to the visualization realm. On the visualization side, they will be applied to the `DataStorage`. As long as all change events are applied, the content of the changed `DataStorage` is the same as if it has been generated from `ComputationStorage` as outlined above.

Regarding parallel access and mutual exclusion, some things are required:

* Generating the events should _not_ lead to any serious delays in the access of `ComputationStorage`. Parallel computation threads should be able to access the content without mentionable restrictions.
* Applying events on the visualization side can be somehow relaxed about order. Actually, each point in the generated graphics data will only be changed _once_ during the whole operation, and that's from "unknown" to some values - and this value is independent of any points and will not change. Therefore, it does not matter in which order the changes are applied, as long as they are _all_ applied.
* On the visualization side, applying events may block access to the whole `DataStorage`.
* Applying events can be single-threaded.
* Read access to the visualization's `DataStorage` for actual drawing of the image should not be delayed for too long due to change event application. If redraw needs to take place _and_ there are events to be applied, the redraw should always have precedence.

### Memory assumptions and maximum event queue size

Implementing storage this way has certain memory implications:

* For each point, storage space has to be reserved in `ComputationStorage`.
* For each point, storage space has to be reserved in `DataStorage`
* For each point, at most _one_ event will be generated and has to be stored in an event queue.

That means that _at most_ three data of constant (and similar) size have to be stored. Assuming computation of an image with 8K resolution, this is about 1.5 GB of storage over all. And this is only needed if visualization does not apply _any_ events until computation is finished.

So, there should not be a storage size problem for any sensible image size and workload.

### Resynchronisation of computation and visualization

If visualization loses track on the change events, it can simply

* throw away the event queue
* start collecting events again from scratch
* ask computation for a new complete `DataStorage`
* apply the collected events after receiving the storage.

It is important that collecting events starts _before_ or by maximum _at the same time_ as generation of the new `DataStorage` so that no events are lost.

## Implementation plan

A possible implementation plan could be:

1. Finalize the design of `ComputationStorage` and `ComputationPlane`. This is about deciding on which Rust language features to use for the needed concurrency.
2. Implement them. This should also include unit tests for the concurrent access.
3. Generate a `ComputationStorage` instead of `DataStorage` in the GUI when the user clicks on "Compute".
4. Change the existing (single-threaded) computation algorithm implementation to use `ComputationStorage` instead of `DataStorage`.
5. Implement `ComputationStorage::initVisualizationStorage` to create a `DataStorage`.
6. Present that `DataStorage` in the GUI.
7. Phase I: Use this code to create the `DataStorage` once at the _end_ of the computation. This gives the current behaviour where the graphics is only shown _after_ computation finishes.
8. Phase II: Use this code to create the `DataStorage` _regulary_ during the computation, e.g. once each second. This creates lots of overhead but allows to implement the update logic on the visualization side.
9. Implement the events described above.
10. Implement generation of the events on the computation side.
11. Implement application of the events on the visualization side.
12. Phase III: Use the event-updated `DataStorage` to update visualization during computation as originally planned.

With these steps being followed, the new storage engine is implemented and ready for further development.

## Organisation details

Source code should be changed in a way that _computation_ data storage and _visualization_ data storage are clearly distinct from each other. Speaking in Java terms, they should be in different packages. Data structures which are used in both storages, like the envisioned `ImageParameters`, should be places in a third "package".

Somewhere in the implementation phase, probably at the beginning or at the end, `DataStorage` should be renamed to `VisualizationStorage`, and the same should happen to `DataPlane`.

## First review results

First review of this plan raised some questions by Claude. These are my thoughts on them:

### Questions and Considerations

> ComputationStage Concurrency: What specific Rust concurrency primitive are you considering? `RwLock<Vec<DataPoint>>`,  `Vec<RwLock<DataPoint>>`, or `Vec<AtomicDataPoint>`? Each has different performance characteristics.

This has indeed to be sorted out. My first impression is that `Vec<RwLock<DataPoint>>` could be what we need.

> Event Granularity: Why limit to single points and horizontal lines? Would rectangular regions be useful for block-based parallel computation algorithms?

See additions to "Change messages from computation to visualization" above.

> Memory Overhead: In high-resolution scenarios (8K), you'll have both ComputationStorage and VisualizationStorage in memory simultaneously. Have you considered the memory implications?

See "Memory assumptions and maximum event queue size" above.

> Event Queue Backpressure: What happens if the visualization thread can't keep up with computation events? Should there be a bounded queue or event coalescing?

See "Memory assumptions and maximum event queue size" above.

### Architectural Suggestion

> Consider whether ImageParameters should be truly shared (Arc) or duplicated. If computation parameters could change mid-computation (zoom, iteration count), sharing might create synchronization issues.

The computation parameters on a `ComputationStorage` will _never_ change after initialisation. If an image with changed parameters is to be derived, some derivation algorithm will be implemented which creates a _new_, independent `ComputationStorage` and initializes it with appropriate data.

But as `ImageParameters` is a small data structure anyway, it won't matter whether it is cloned or shared between computation and visualization storage.

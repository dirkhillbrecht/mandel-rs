// Two-dimensional data plane for the data points of mandel-rs

use crate::DataPoint;

#[derive(Debug)]
pub struct DataPlane {
    data: Vec<Option<DataPoint>>,
    width: usize,
    height: usize,
}

impl DataPlane {
    pub fn new(width: usize, height: usize) -> DataPlane {
        DataPlane { data: vec![None; width*height] , width, height }
    }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    fn index(&self,x:usize,y:usize) -> usize { y*self.width+x }
    pub fn get(&self, x: usize, y: usize) -> Option<&DataPoint> {
        self.data[self.index(x,y)].as_ref()
    }
    pub fn set(&mut self, x: usize, y: usize, data_point: DataPoint) {
        let idx=self.index(x,y);
        self.data[idx] = Option::Some(data_point);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct() {
        let ds: DataPlane = DataPlane::new(50,60);
        assert_eq!(ds.width(),50);
        assert_eq!(ds.height(),60);
    }

    #[test]
    fn test_get_and_set() {
        let mut plane: DataPlane = DataPlane::new(2,2);
        let point: DataPoint = DataPoint::new(14,7.0,9.0);
        plane.set(0, 0, point);
        //let oretrieved=ds.get(0,0).unwrap();
        //assert_eq!(oretrieved.is_some(),true);
        let retrieved=plane.get(0,0).unwrap();
        assert_eq!(retrieved.iteration_count(),14);
        assert_eq!(retrieved.final_x(),7.0);
        assert_eq!(retrieved.final_y(),9.0);
    }

}

// end of file

// Two-dimensional data plane for the data points of mandel-rs

use crate::storage::data_point::DataPoint;

#[derive(Debug, Clone)]
pub struct DataPlane {
    data: Vec<Option<DataPoint>>,
    width: usize,
    height: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    dotsize: f64,
}

impl DataPlane {
    pub fn new(width: usize, height: usize, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> DataPlane {
        let x_dotsize=(x_max-x_min)/width as f64;
        let y_dotsize=(y_max-y_min)/height as f64;
        if (x_dotsize-y_dotsize).abs()<1e-5 {
            let calc_shift=x_dotsize/2.0;
            DataPlane { data: vec![None; width*height], width, height,
                x_min: x_min+calc_shift, x_max: x_max+calc_shift, y_min: y_min+calc_shift, y_max: y_max+calc_shift,
                dotsize: x_dotsize
            }
        }
        else {
            let dotsize=x_dotsize.max(y_dotsize);
            let x_center=(x_max+x_min+dotsize)/2.0;
            let y_center=(y_max+y_min+dotsize)/2.0;
            let x_dist=dotsize*((width as f64)/2.0);
            let y_dist=dotsize*((height as f64)/2.0);
            DataPlane { data: vec![None; width*height], width, height,
                x_min: x_center-x_dist, x_max: x_center+x_dist, y_min: y_center-y_dist, y_max: y_center+y_dist,
                dotsize
            }
        }
    }
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn x_min(&self) -> f64 { self.x_min }
    pub fn x_max(&self) -> f64 { self.x_max }
    pub fn y_min(&self) -> f64 { self.y_min }
    pub fn y_max(&self) -> f64 { self.y_max }
    pub fn x(&self,x_pix:usize) -> f64 { self.x_min+x_pix as f64*self.dotsize }
    pub fn y(&self,y_pix:usize) -> f64 { self.y_max-y_pix as f64*self.dotsize }
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
        let ds: DataPlane = DataPlane::new(50,60,-2.0,2.0,-1.0,1.0);
        assert_eq!(ds.width(),50);
        assert_eq!(ds.height(),60);
    }

    #[test]
    fn test_get_and_set() {
        let mut plane: DataPlane = DataPlane::new(2,2,-2.0,2.0,-1.0,1.0);
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

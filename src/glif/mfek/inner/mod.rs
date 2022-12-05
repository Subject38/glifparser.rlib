pub mod cubic;
pub mod quad;

use serde::{Serialize, Deserialize};
use crate::{PointType, PointData, contour::State};
use self::{cubic::MFEKCubicInner, quad::MFEKQuadInner};

use super::{contour::{MFEKContourCommon, MFEKContourCommonIterator, MFEKCommonMismatchError}, point::MFEKPointCommon};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MFEKContourInner<PD: PointData> {
    Cubic(MFEKCubicInner<PD>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum MFEKContourInnerType {
    Cubic,
}

pub trait MFEKCommonInner<PD: PointData> {
    fn sub(&self, start_index: usize, end_index: usize) -> MFEKContourInner<PD>;
    fn append(&mut self, other: &MFEKContourInner<PD>) -> Result<(), MFEKCommonMismatchError>;
}

impl<PD: PointData> MFEKContourInner<PD> {
    pub fn iter(&self) -> MFEKContourCommonIterator<'_, PD>{
        return MFEKContourCommonIterator { index: 0, contour: self }
    }
}

impl<PD: PointData> MFEKCommonInner<PD> for MFEKContourInner<PD> {
    fn sub(&self, start_index: usize, end_index: usize) -> MFEKContourInner<PD> {
        match self {
            MFEKContourInner::Cubic(contour) => {
                let mut sub_contour:Vec<crate::Point<PD>> = Vec::new();

                for i in start_index..end_index {
                    sub_contour.push(contour[i].clone());
                }

                MFEKContourInner::Cubic(sub_contour)
            }
        }
    }

    fn append(&mut self, other: &MFEKContourInner<PD>) -> Result<(), MFEKCommonMismatchError> {
        match self {
            MFEKContourInner::Cubic(contour) => {
                if let Some(other_cubic) = other.cubic() {
                    for point in other_cubic {
                        contour.push(point.clone());
                    }
                    Ok(())
                } else {
                    Err(MFEKCommonMismatchError)
                }
            },
        }
    }
}

impl<PD: PointData> MFEKContourCommon<PD> for MFEKContourInner<PD> {
    fn len(&self) -> usize {
        match self {
            MFEKContourInner::Cubic(contour) => contour.len()
        }
    }

    fn first(&self) -> &dyn MFEKPointCommon<PD> {
        return self.get_point(0).unwrap();
    }

    fn last(&self) -> &dyn MFEKPointCommon<PD> {
        return self.get_point(self.len() - 1).unwrap();
    }

    fn is_open(&self) -> bool {
        match self {
            MFEKContourInner::Cubic(contour) => contour.is_open()
        }
    }

    fn is_closed(&self) -> bool {
        !self.is_open()
    }

    fn reverse(&mut self) {
        match self {
            MFEKContourInner::Cubic(contour) => {contour.reverse()}
        }
    }

    fn delete(&mut self, index: usize) {
        match self {
            MFEKContourInner::Cubic(contour) => {contour.remove(index);}
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            MFEKContourInner::Cubic(contour) => contour.is_empty(),
        }
    }

    fn set_open(&mut self) {
        match self {
            MFEKContourInner::Cubic(contour) => contour[0].ptype = PointType::Move,
        }
    }

    fn set_closed(&mut self) {
        match self {
            MFEKContourInner::Cubic(contour) => contour[0].ptype = PointType::Curve,
        }
    }

    fn get_point_mut(&mut self, pidx: usize) -> Option<&mut dyn MFEKPointCommon<PD>>{
        match self {
            MFEKContourInner::Cubic(contour) => {           
                if let Some(point) = contour.get_mut(pidx) {
                    Some(point)
                } else {
                    None
                }
            },
        }
    }

    fn get_point(&self, pidx: usize) -> Option<&dyn MFEKPointCommon<PD>> {
        match self {
            MFEKContourInner::Cubic(contour) => {
                if let Some(point) = contour.get(pidx) {
                    Some(point)
                } else {
                    None
                }
            },
        }    
    }

    fn get_type(&self) -> MFEKContourInnerType {
        match self {
            MFEKContourInner::Cubic(_) => MFEKContourInnerType::Cubic,
        }    
    }

    fn cubic(&self) -> Option<&MFEKCubicInner<PD>> {
        match self {
            MFEKContourInner::Cubic(c) => Some(c),
            _ => None,
        }
    }

    fn cubic_mut(&mut self) -> Option<&mut MFEKCubicInner<PD>> {
        match self {
            MFEKContourInner::Cubic(c) => Some(c),
            _ => None,
        }    }
    
}
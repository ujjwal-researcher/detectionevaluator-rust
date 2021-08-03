use crate::dtypes::{BOXDATA, FVECTOR, IVECTOR};
use std::{vec};

pub enum BOXFORMAT{
    XYWH,
    XYXY
}

pub trait BoxCollection{
    fn heights() -> &FVECTOR;
    fn widths() -> &FVECTOR;
    fn areas () -> &FVECTOR;
    fn aspect_ratios() -> &FVECTOR;
    fn boxes() -> &BOXDATA;
    fn confidences() -> &FVECTOR;
    fn category_ids() -> &IVECTOR;
    fn is_normalized() -> &bool;
    fn image_name() -> &str;
}

pub struct BoxCollectionXYWH{
    image_file_name : str,
    box_type : BOXFORMAT,
    bounding_box : BOXDATA,
    box_confidences : FVECTOR,
    box_category_ids : IVECTOR,
    box_normalized : bool
}

pub struct BoxCollectionXYXY{
    image_file_name : str,
    box_type : BOXFORMAT,
    bounding_box : BOXDATA,
    box_confidences : FVECTOR,
    box_category_ids : IVECTOR,
    box_normalized : bool
}

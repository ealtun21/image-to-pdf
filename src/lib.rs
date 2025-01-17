use printpdf::{
    image_crate::{DynamicImage, GenericImageView},
    Error, Image, ImageTransform, PdfDocument, PdfDocumentReference, Px,
};
use rayon::prelude::ParallelExtend;

use std::{
    convert::TryInto,
    io::{prelude::*, BufWriter},
};

pub struct ImageToPdf {
    images: Vec<DynamicImage>,
    dpi: f64,
    document_title: String,
}

impl Default for ImageToPdf {
    fn default() -> Self {
        Self {
            images: Vec::new(),
            dpi: 300.0,
            document_title: "".to_string(),
        }
    }
}

impl ImageToPdf {
    pub fn new(images: Vec<DynamicImage>, dpi: f64, document_title: String) -> ImageToPdf {
        ImageToPdf {
            images,
            dpi,
            document_title,
        }
    }

    /// Add an image to the PDF output.
    pub fn add_image(mut self, image: DynamicImage) -> ImageToPdf {
        self.images.push(image);
        self
    }

    /// Add one or more images to the PDF output in parallel.
    pub fn add_images_par(
        mut self,
        images: impl rayon::iter::ParallelIterator<Item = DynamicImage>,
    ) -> ImageToPdf {
        self.images.par_extend(images);
        self
    }

    /// Add one or more images to the PDF output.
    pub fn add_images(mut self, images: impl IntoIterator<Item = DynamicImage>) -> ImageToPdf {
        self.images.extend(images);
        self
    }

    /// Set the DPI scaling of the PDF output.
    pub fn set_dpi(mut self, dpi: f64) -> ImageToPdf {
        self.dpi = dpi;
        self
    }

    /// Sets the title of the PDF output.
    pub fn set_document_title(mut self, document_title: impl Into<String>) -> ImageToPdf {
        self.document_title = document_title.into();
        self
    }

    /// Writes the PDF output to `out`.
    pub fn create(self) -> PdfDocumentReference {
        let dpi = self.dpi;
        let doc = PdfDocument::empty(self.document_title);
        self.images
            .into_iter()
            .for_each(|image| add_page(image, &doc, dpi));
        doc
    }

    /// Writes the PDF output to `out`.
    pub fn save(
        self,
        doc: PdfDocumentReference,
        out: &mut BufWriter<impl Write>,
    ) -> Result<(), Error> {
        doc.save(out)
    }
}

#[cfg(feature = "progress")]
pub mod webp {
    use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
    use std::io::{BufWriter, Write};

    use printpdf::{Error, PdfDocument, PdfDocumentReference};

    use crate::{add_page, ImageToPdf};

    impl ImageToPdf {
        pub fn create_with_progress(
            self,
            sty: ProgressStyle,
            m: MultiProgress,
        ) -> PdfDocumentReference {
            let pb = ProgressBar::new(self.images.len() as u64).with_style(sty);
            m.add(pb.clone());

            let dpi = self.dpi;
            let doc = PdfDocument::empty(self.document_title);
            self.images.into_iter().for_each(|image| {
                pb.inc(1);
                add_page(image, &doc, dpi);
            });
            m.clear().unwrap();
            doc
        }
    }
}

fn add_page(image: DynamicImage, doc: &PdfDocumentReference, dpi: f64) {
    let (width, height) = image.dimensions();
    let (width, height) = (width.try_into().unwrap(), height.try_into().unwrap());

    let (page, layer) = doc.add_page(
        Px(width).into_pt(dpi).into(),
        Px(height).into_pt(dpi).into(),
        "",
    );

    let image = Image::from_dynamic_image(&image);
    let current_layer = doc.get_page(page).get_layer(layer);

    image.add_to_layer(current_layer, ImageTransform::default());
}

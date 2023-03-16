//! Defines the [PdfPageSquigglyAnnotation] struct, exposing functionality related to a single
//! user annotation of type `PdfPageAnnotationType::Squiggly`.

use crate::bindgen::{FPDF_ANNOTATION, FPDF_DOCUMENT, FPDF_PAGE};
use crate::bindings::PdfiumLibraryBindings;
use crate::page_annotation_objects::PdfPageAnnotationObjects;
use crate::page_annotation_private::internal::PdfPageAnnotationPrivate;

/// A single `PdfPageAnnotation` of type `PdfPageAnnotationType::Squiggly`.
pub struct PdfPageSquigglyAnnotation<'a> {
    handle: FPDF_ANNOTATION,
    objects: PdfPageAnnotationObjects<'a>,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfPageSquigglyAnnotation<'a> {
    pub(crate) fn from_pdfium(
        document_handle: FPDF_DOCUMENT,
        page_handle: FPDF_PAGE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfPageSquigglyAnnotation {
            handle: annotation_handle,
            objects: PdfPageAnnotationObjects::from_pdfium(
                document_handle,
                page_handle,
                annotation_handle,
                bindings,
            ),
            bindings,
        }
    }
}

impl<'a> PdfPageAnnotationPrivate<'a> for PdfPageSquigglyAnnotation<'a> {
    #[inline]
    fn handle(&self) -> FPDF_ANNOTATION {
        self.handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }

    #[inline]
    fn objects_impl(&self) -> &PdfPageAnnotationObjects {
        &self.objects
    }

    #[inline]
    fn objects_mut_impl(&mut self) -> &mut PdfPageAnnotationObjects<'a> {
        &mut self.objects
    }
}

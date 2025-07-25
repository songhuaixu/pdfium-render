use pdfium_render::prelude::*;

pub fn main() -> Result<(), PdfiumError> {
    // For general comments about pdfium-render and binding to Pdfium, see export.rs.

    let pdfium = Pdfium::default();

    let mut document = pdfium.load_pdf_from_file("test/text-test.pdf", None)?; // Load the sample file...

    // Delete all pages in the document except the first.

    for index in (1..document.pages().len()).rev() {
        document.pages().get(index)?.delete()?;
    }

    // Create a new page that will serve as the destination for our moved page objects.

    document
        .pages_mut()
        .create_page_at_end(PdfPagePaperSize::a4())?;

    // Move all the page objects on the bottom half of the first page to the destination page.

    let source_page = document.pages().first()?;

    let source_objects = source_page.objects().create_group(|object| {
        object
            .bounds()
            .map(|bounds| {
                // Only select objects on the bottom half of the page.

                bounds.top() < source_page.height() / 2.0
            })
            .unwrap_or(false)
    })?;

    println!("{} objects selected on page", source_objects.len());

    let mut destination_page = document.pages().last()?;

    source_objects.move_to_page(&mut destination_page)?;

    println!("{} objects moved to page", destination_page.objects().len());

    document.save_to_file("test/copy-test.pdf")
}

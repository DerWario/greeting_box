use escpos::printer::Printer;
use escpos::ui::line::{LineBuilder, LineStyle};
use escpos::utils::*;
use escpos::{driver::*, errors::Result};

pub struct PrintMetaData {
    pub global_print_count: u32,
    pub global_message_count: u32,
    pub message_number: u32,
    pub message_print_count: u32,
    pub written_at: String,
}
pub struct PrintData {
    pub print_meta_data: PrintMetaData,
    pub title: Option<String>,
    pub author: Option<String>,
    pub message: String,
}

pub fn print(print_data: PrintData) -> Result<()> {
    let driver = UsbDriver::open(0x04b8, 0x0202, None, None)?;
    let mut printer = Printer::new(driver, Protocol::default(), None);

    // Line
    let simple_line = LineBuilder::new().style(LineStyle::Simple).build();

    // Name + address
    printer
        .init()?
        .page_code(PageCode::ISO8859_15)?
        .justify(JustifyMode::LEFT)?
        .writeln(
            format!(
                "format!(Druck Nr. : {}",
                print_data.print_meta_data.global_print_count
            )
            .as_str(),
        )?
        .writeln(
            format!(
                "Nachricht: {}/{}",
                print_data.print_meta_data.message_number,
                print_data.print_meta_data.global_message_count
            )
            .as_str(),
        )?
        .writeln(
            format!(
                "Nachricht gedruckt: {} mal",
                print_data.print_meta_data.message_print_count
            )
            .as_str(),
        )?
        .writeln(
            format!(
                "Eingetragen am: {} Uhr",
                print_data.print_meta_data.written_at
            )
            .as_str(),
        )?
        .cut()?
        .size(2, 2)?
        .bold(true)?;

    if let Some(title) = &print_data.title {
        printer.writeln(title)?;
        printer.draw_line(simple_line)?;
    }
    printer.reset_size()?;
    printer.bold(false)?;

    for line in print_data.message.lines() {
        printer.writeln(line)?;
    }

    printer.feed()?;
    if let Some(author) = &print_data.author {
        printer.writeln(format!("- {}", author).as_str())?;
    }

    printer.print_cut()?;

    Result::Ok(())
}

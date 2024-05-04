// // use soap::{Error, Fault, SoapVersion, Transport};
// // use xml::{reader::XmlReader, writer::XmlWriter};
// // use xml::Event;
// use soap::Fault;
// use xml::writer::{EventWriter, XmlEvent};

// #[derive(Debug, Default)]
// struct Request {
//     method: String,
//     path: String,
//     body: String,
//     // Другие поля запроса, если необходимо
// }

// #[derive(Debug, Default)]
// struct Response {
//     status_code: u16,
//     body: String,
//     headers: Vec<(String, String)>,
//     // Другие поля ответа, если необходимо
// }

// impl Request {
//     fn to_xml<W: std::io::Write>(
//         &self,
//         writer: &mut EventWriter<W>,
//     ) -> Result<(), xml::writer::Error> {
//         // Начинаем запись XML-элемента "Request"
//         writer.write(XmlEvent::start_element("Request"))?;

//         // Записываем поля запроса как дочерние элементы
//         writer.write(XmlEvent::start_element("Method"))?;
//         writer.write(XmlEvent::characters(&self.method))?;
//         writer.write(XmlEvent::end_element())?;

//         writer.write(XmlEvent::start_element("Path"))?;
//         writer.write(XmlEvent::characters(&self.path))?;
//         writer.write(XmlEvent::end_element())?;

//         writer.write(XmlEvent::start_element("Body"))?;
//         writer.write(XmlEvent::characters(&self.body))?;
//         writer.write(XmlEvent::end_element())?;

//         // Завершаем запись XML-элемента "Request"
//         writer.write(XmlEvent::end_element())?;

//         Ok(())
//     }
// }

// impl Response {
//     fn to_xml<W: std::io::Write>(
//         &self,
//         writer: &mut EventWriter<W>,
//     ) -> Result<(), xml::writer::Error> {
//         // Начинаем запись XML-элемента "Response"
//         writer.write(XmlEvent::start_element("Response"))?;

//         // Записываем поля ответа как дочерние элементы
//         writer.write(XmlEvent::start_element("StatusCode"))?;
//         writer.write(XmlEvent::characters(&self.status_code.to_string()))?;
//         writer.write(XmlEvent::end_element())?;

//         writer.write(XmlEvent::start_element("Body"))?;
//         writer.write(XmlEvent::characters(&self.body))?;
//         writer.write(XmlEvent::end_element())?;

//         // Записываем заголовки ответа
//         for (name, value) in &self.headers {
//             writer.write(XmlEvent::start_element("Header"))?;
//             writer.write(XmlEvent::start_element("Name"))?;
//             writer.write(XmlEvent::characters(name))?;
//             writer.write(XmlEvent::end_element())?;
//             writer.write(XmlEvent::start_element("Value"))?;
//             writer.write(XmlEvent::characters(value))?;
//             writer.write(XmlEvent::end_element())?;
//             writer.write(XmlEvent::end_element())?;
//         }

//         // Завершаем запись XML-элемента "Response"
//         writer.write(XmlEvent::end_element())?;

//         Ok(())
//     }
// }

// fn handle_request<'a>(reader: &mut XmlReader<'_>, writer: &mut XmlWriter<'a>) -> Result<(), Error> {
//     let request = Request::from_xml(reader).map_err(Fault::from)?;
//     let response = Response::default();

//     // generate response
//     response.to_xml(writer).map_err(Fault::from)?;
//     Ok(())
// }

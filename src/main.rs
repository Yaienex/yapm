use zip::write::SimpleFileOptions;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::str::FromStr;
use std::{env, vec};
use std::process::{exit, Command};
use lopdf::{Document, Object, ObjectId, Bookmark};


fn main() {
    //getting the argument and removing the 0th arg (program name)
    let mut args:Vec<String> = env::args().collect(); args.reverse(); args.pop();args.reverse();
    if args.len() == 0{
        println!("No arguments were given, type yapm help to get the list of the possible command");
        exit(1);
    }
    let command = args[0].as_str();
    let mut option_enc = false;
    match command{
        "merge" | "m" => {
            println!("Merging the given pdf in the given order ");
            let mut name = "merged_documents.pdf";
            let mut documents = Vec::new();
            let mut doc;
            let mut pdf_name;
            
            for i in 1..args.len(){
                pdf_name = &args[i];
                if option_enc{
                    option_enc = false;
                    continue;
                }
                if pdf_name == "-o"{
                    name = args[i+1].as_str();
                    option_enc = true;
                    continue;
                }
                doc = match Document::load(pdf_name.clone()).as_mut(){
                    Ok(e) =>e,
                    Err(err) => {println!("Error while trying to load the pdf {pdf_name}:\n {err}");exit(1)},
                }.clone();
                documents.push(doc);
            }
            let _ = merge(documents,name);
        },
        "help" |"h" |"?" => help(),
        "get" | "g" =>{let document = Document::load(&args[1]).unwrap(); get_page(document,args)},
        "del" |"d" => {let document = Document::load(&args[1]).unwrap();  del_page(document,args)},
        "split" |"s" => {let document = Document::load(&args[1]).unwrap();split(document,args)},
        _ => print!("No command was recognized. type yapm help to get all the commands")
    }

}

fn split(document: Document,_args:Vec<String>) {
    //Doc req
    let count = document.get_pages().len();
    let mut pages_numbers:Vec<u32> = Vec::new();
    let mut doc ;
    let mut name;
    let mut files: Vec<PathBuf> = vec![];
    for i in 1..=count{
        doc = document.clone();
        for j in 1..=count{
            if j != i {
            pages_numbers.push(j as u32);
            }
        }
        doc.delete_pages(&pages_numbers);
        name = format!("page_{i}.pdf");
        files.push(PathBuf::from(&name));
        doc.save(name).unwrap();
        pages_numbers.clear();
    }

    let archive_path = "splitted_document.zip";
    let archive = PathBuf::from_str(archive_path).unwrap();
    let existing_zip = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(archive)
        .unwrap();

    let mut append_zip = zip::ZipWriter::new(existing_zip);

    for file in &files {
        append_zip
            .start_file(file.to_string_lossy(), SimpleFileOptions::default())
            .unwrap();

        let mut f = File::open(file).unwrap();
        let _ = std::io::copy(&mut f, &mut append_zip);
    }

    append_zip.finish().unwrap();
    
    delete_files(files.clone());

}

fn merge(documents:Vec<Document>,name: &str)-> std::io::Result<()> {
    // Define a starting `max_id` (will be used as start index for object_ids).
    let mut max_id = 1;
    let mut pagenum = 1;
    // Collect all Documents Objects grouped by a map
    let mut documents_pages = BTreeMap::new();
    let mut documents_objects = BTreeMap::new();
    let mut document = Document::with_version("1.5");

    for mut doc in documents {
        let mut first = false;
        doc.renumber_objects_with(max_id);

        max_id = doc.max_id + 1;

        documents_pages.extend(
            doc
                    .get_pages()
                    .into_iter()
                    .map(|(_, object_id)| {
                        if !first {
                            let bookmark = Bookmark::new(String::from(format!("Page_{}", pagenum)), [0.0, 0.0, 1.0], 0, object_id);
                            document.add_bookmark(bookmark, None);
                            first = true;
                            pagenum += 1;
                        }

                        (
                            object_id,
                            doc.get_object(object_id).unwrap().to_owned(),
                        )
                    })
                    .collect::<BTreeMap<ObjectId, Object>>(),
        );
        documents_objects.extend(doc.objects);
    }

    // "Catalog" and "Pages" are mandatory.
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    // Process all objects except "Page" type
    for (object_id, object) in documents_objects.iter() {
        // We have to ignore "Page" (as are processed later), "Outlines" and "Outline" objects.
        // All other objects should be collected and inserted into the main Document.
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                // Collect a first "Catalog" object and use it for the future "Pages".
                catalog_object = Some((
                    if let Some((id, _)) = catalog_object {
                        id
                    } else {
                        *object_id
                    },
                    object.clone(),
                ));
            }
            b"Pages" => {
                // Collect and update a first "Pages" object and use it for the future "Catalog"
                // We have also to merge all dictionaries of the old and the new "Pages" object
                if let Ok(dictionary) = object.as_dict() {
                    let mut dictionary = dictionary.clone();
                    if let Some((_, ref object)) = pages_object {
                        if let Ok(old_dictionary) = object.as_dict() {
                            dictionary.extend(old_dictionary);
                        }
                    }

                    pages_object = Some((
                        if let Some((id, _)) = pages_object {
                            id
                        } else {
                            *object_id
                        },
                        Object::Dictionary(dictionary),
                    ));
                }
            }
            b"Page" => {}     // Ignored, processed later and separately
            b"Outlines" => {} // Ignored, not supported yet
            b"Outline" => {}  // Ignored, not supported yet
            _ => {
                document.objects.insert(*object_id, object.clone());
            }
        }
    }

    // If no "Pages" object found, abort.
    if pages_object.is_none() {
        println!("Pages root not found.");

        return Ok(());
    }

    // Iterate over all "Page" objects and collect into the parent "Pages" created before
    for (object_id, object) in documents_pages.iter() {
        if let Ok(dictionary) = object.as_dict() {
            let mut dictionary = dictionary.clone();
            dictionary.set("Parent", pages_object.as_ref().unwrap().0);

            document
                    .objects
                    .insert(*object_id, Object::Dictionary(dictionary));
        }
    }

    // If no "Catalog" found, abort.
    if catalog_object.is_none() {
        println!("Catalog root not found.");

        return Ok(());
    }

    let catalog_object = catalog_object.unwrap();
    let pages_object = pages_object.unwrap();

    // Build a new "Pages" with updated fields
    if let Ok(dictionary) = pages_object.1.as_dict() {
        let mut dictionary = dictionary.clone();

        // Set new pages count
        dictionary.set("Count", documents_pages.len() as u32);

        // Set new "Kids" list (collected from documents pages) for "Pages"
        dictionary.set(
            "Kids",
            documents_pages
                    .into_iter()
                    .map(|(object_id, _)| Object::Reference(object_id))
                    .collect::<Vec<_>>(),
        );

        document
                .objects
                .insert(pages_object.0, Object::Dictionary(dictionary));
    }

    // Build a new "Catalog" with updated fields
    if let Ok(dictionary) = catalog_object.1.as_dict() {
        let mut dictionary = dictionary.clone();
        dictionary.set("Pages", pages_object.0);
        dictionary.remove(b"Outlines"); // Outlines not supported in merged PDFs

        document
                .objects
                .insert(catalog_object.0, Object::Dictionary(dictionary));
    }

    document.trailer.set("Root", catalog_object.0);

    // Update the max internal ID as wasn't updated before due to direct objects insertion
    document.max_id = document.objects.len() as u32;

    // Reorder all new Document objects
    document.renumber_objects();

    // Set any Bookmarks to the First child if they are not set to a page
    document.adjust_zero_pages();

    // Set all bookmarks to the PDF Object tree then set the Outlines to the Bookmark content map.
    if let Some(n) = document.build_outline() {
        if let Ok(Object::Dictionary(dict)) = document.get_object_mut(catalog_object.0) {
            dict.set("Outlines", Object::Reference(n));
        }
    }

    document.compress();

    document.save(name).unwrap();


    Ok(())
}

fn help(){
    println!("In progress");
}

fn del_page(document:Document,args:Vec<String>){
    let mut doc = document;
    doc.delete_pages(&[args[2].parse::<u32>().unwrap()]);
    let name = format!("modified_{}",args[1]);
    doc.save(name).unwrap();
}
fn get_page(document: Document,args:Vec<String>){
    let count = document.get_pages().len();
    let mut pages_numbers:Vec<u32> = Vec::new();
    let mut doc = document;
    let page = args[2].parse::<usize>().unwrap();
    for j in 1..=count{
        if j != page {
            pages_numbers.push(j as u32);
        }
    }
    doc.delete_pages(&pages_numbers);
    let name = format!("Page_{page}_{}",args[1]);
    doc.save(name).unwrap();
}

fn delete_files(files: Vec<PathBuf>){
    let mut cmd;
    let mut filename;
    for file in files{
        filename = file.to_str().unwrap();
        cmd = format!("rm {filename}");
        Command::new("sh")
        .arg("-c")
        .arg(cmd).output().expect("Failed to delete temporary files");
    }
}
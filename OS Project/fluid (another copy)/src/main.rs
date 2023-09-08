use fltk::{prelude::*,*,};
use fltk::enums::FrameType;
use fltk::enums::Font;
use fltk::text::TextDisplay;
use walkdir::WalkDir;
use std::fs;
mod myuifile;
use fltk::group::Scroll;
use fltk::tree::TreeItem;
use fltk::window::MenuWindow;
use fltk::misc::Progress;
use fltk::enums::Color;
extern crate glob;
use glob::glob;
use std::error::Error;
use std::path::Path;
use std::fs::File;
extern crate walkdir;
//use std::error::Error;
use rand::Rng;
extern crate fs_extra;
use fs_extra::dir::get_size;
fn main() {

    let app= app::App::default();
    
    app::set_visible_focus(false);
 
let mut ui = myuifile::UserInterface2::entry_window();
let mut mainwin = ui.mainwin.clone();
let mut analyze = ui.analyze.clone();
let mut foldin = ui.foldin.clone();
//analyze.set_frame(RoundedBox);
//mainwin.end();


mainwin.show();
let mut file_names = Vec::new();
let mut file_sizes: Vec<f64> = Vec::new();
let mut file_names2:Vec<String> = Vec::new();
let mut file_sizes2: Vec<f64> = Vec::new();

let mut ui2 = myuifile::UserInterface3::second_window();
let mut secondwin = ui2.secondwin.clone();
secondwin.hide();
let mut checking = true;
   // secondwin.make_resizable(true);
    //secondwin.deactivate();

    ui.analyze.set_callback(move |_| {
        //let mut checking = mainwin.takes_events();
        mainwin.hide();
        

let mut folder  = foldin.value().replace("file://","") ;
let mut tempString: String;
let mut x: f64;
let mut folder2 = foldin.value().replace("file://","") ;
for file in WalkDir::new(folder).into_iter().filter_map(|file| file.ok()){
    tempString = file.path().display().to_string();
    //x = fs::metadata(&tempString).expect("Could not obtain file size!").len() as f64;
    x = get_size(&tempString).unwrap() as f64;
    file_names2.push(tempString.replace(&(foldin.value()+"/"),""));
    file_sizes2.push(x);
};
for file in fs::read_dir(folder2).unwrap(){
    tempString = file.unwrap().path().display().to_string();
    //x = fs::metadata(&tempString).expect("Could not obtain file size!").len() as f64;
    x = get_size(&tempString).unwrap() as f64;
    file_names.push(tempString.replace(&( foldin.value() + "/"),""));
    file_sizes.push(x);
}
let mut trees = ui2.maintree.clone();
let mut root =trees.root();
trees.set_root_label("Local Drive");
let mut i = 0;
while i< file_names2.len(){
    trees.add(&file_names2[i]);
    i= i+1;
}

let mut filegroup = ui2.filegroup.clone();
//let mut sizegroup = ui2.sizegroup.clone();
 



let mut piewin = window::DoubleWindow::new(50,50,500,500,"pie chart");
piewin.make_resizable(true);
piewin.set_color(Color::by_index(39));

let mut chart = misc::Chart::new(0,0,500,500,"Pie Chart Representation");
chart.set_type(misc::ChartType::Pie);
chart.set_bounds(0.0, 1.0);
chart.set_text_size(12);
chart.set_color(Color::by_index(39));

chart.clear();
let mut i = 0;
let mut j = 1000;
let mut tot = 0.0;
// Calculating Total Sizw of Current Directory Being Analyzed
while i < file_sizes.len(){
tot = tot + file_sizes[i]; 
i = i + 1;      
}
println!("{}",tot);
i=0;

let mut it = 0;
let mut y0 = 50;

while it < file_sizes.len(){
    let mut fileoutput = output::Output::new(190, y0, 350, 25, None);
    let mut filesizeoutput = output::Output::new(560, y0, 125, 25, None);
    let mut shortened =foldin.value().replace("file://","");
    let slash = "/";
    let mut shortened2 =file_names[it].replace(&(shortened.to_owned() + &slash.to_owned()),"");
    
    fileoutput.set_value(&shortened2);
    fileoutput.set_selection_color(Color::by_index(132));
        fileoutput.set_frame(FrameType::RoundUpBox);
        fileoutput.set_label_font(Font::by_index(13));
        fileoutput.set_label_color(Color::by_index(92));
       
    let mut mb = " KB";
    let mut sizeinmb = file_sizes[it] / 1000.00;
    filesizeoutput.set_value(&(sizeinmb.to_string().to_owned() + &mb.to_string().to_owned()));
    filesizeoutput.set_selection_color(Color::by_index(132));
    filesizeoutput.set_frame(FrameType::RoundUpBox);
    filesizeoutput.set_label_font(Font::by_index(13));
    filesizeoutput.set_label_color(Color::by_index(92));
       
    filegroup.add(&fileoutput);
    filegroup.add(&filesizeoutput);
    it = it + 1;     
    y0 = y0 +27; 
    }
    let mut fileoutput2 = output::Output::new(190, y0, 350, 25, None);
    let mut filesizeoutput2 = output::Output::new(560, y0, 125, 25, None);
    
    fileoutput2.set_color(Color::by_index(92));
    fileoutput2.set_selection_color(Color::by_index(132));
    fileoutput2.set_frame(FrameType::RoundUpBox);
    fileoutput2.set_label_font(Font::by_index(13));
    fileoutput2.set_label_color(Color::by_index(92));
   
    filesizeoutput2.set_color(Color::by_index(92));
    filesizeoutput2.set_selection_color(Color::by_index(132));
    filesizeoutput2.set_frame(FrameType::RoundUpBox);
    filesizeoutput2.set_label_font(Font::by_index(13));
    filesizeoutput2.set_label_color(Color::by_index(92));
       
    
fileoutput2.set_value("Total Disk Size");
let mut kb = " KB";
let mut correct = &(tot.to_string().to_owned() + &kb.to_owned());
filesizeoutput2.set_value(correct);
    filegroup.add(&fileoutput2);
    filegroup.add(&filesizeoutput2);

    secondwin.end();
    secondwin.show();
    let mut otherFilesTot = 0.0; // Variable to store the size of the remaining files
    let mut percentage = 0.0;
    let mut rng = rand::thread_rng();
    let mut randomColor: u32 = rng.gen();
// Assigning size according to currentFileSize/SummationOfSizes
while i < file_names.len(){
    percentage = file_sizes[i]/tot; 
          if percentage < 0.025 {percentage = 0.025};
          if percentage > 0.8 {percentage = 0.8}; 
	  
	  randomColor = rng.gen();
	    //i = i + 1;
        
  //chart.add(file_size, file_name, color);
  let mut shortened =foldin.value().replace("file://","");
  let slash = "/";
  let mut shortened2 =file_names[i].replace(&(shortened.to_owned() + &slash.to_owned()),"");
  chart.add(percentage , &shortened2, enums::Color::from_u32(randomColor));


  //chart.add(file_sizes[i]/tot, &shortened2, enums::Color::from_u32(j));
print!("{} - ", file_sizes[i]/tot);
i = i + 1;
//j = j + 10000;

}

//file_names.clear();

chart.show();
piewin.end();
//piewin.show();
let mut piech = ui2.showpie.clone();

ui2.showpie.set_callback(move |_| piewin.show());

let mut barbut = ui2.showbar.clone();

let mut barwin = window::DoubleWindow::new(0,0,500,500,"Percentage Bar");
barwin.set_color(Color::by_index(39));
barwin.hide();
barwin.make_resizable(true);
secondwin.set_color(Color::by_index(39));

let  mut k = 0;
let mut x = 0;
let mut y = 0 ;
let mut bargroup = Scroll::new(0, 0, 700, 500, None);
while k< file_names.len(){
    let mut bar = Progress::new(x, y, 750, 25, None);
	bar.set_color(Color::by_index(22));
    bar.set_minimum(0.0);
 bar.set_maximum(tot);
 bar.set_value(file_sizes[k]);
 bar.set_selection_color(Color::by_index(92));
 //bar.set_selection_color(Color::by_index(92));
 let mut shortened =foldin.value().replace("file://","");
    let slash = "/";
    let mut shortened2 =file_names[k].replace(&(shortened.to_owned() + &slash.to_owned()),"");
let mut percent:i8 = (((file_sizes[k] / tot) *100.0) as i8);
    bar.set_label(&(shortened2.to_owned() + "   " +&percent.to_string().to_owned() + "%"));
    k = k +1;
    y=y +25;
    //bar.show();
    bargroup.add(&bar);
}
barwin.end(); 
barbut.set_callback(move |_| {barwin.show();});
let mut inval = foldin.value(); 
let openext = ui2.exten.clone();

ui2.exten.set_callback(move |_| {
let mut extwin = window::DoubleWindow::new(0,0,300,300,"Search by Extensions");
extwin.set_color(Color::by_index(39));
let mut extbut = button::Button::new(100,140,90,25,"Find Extension");
extbut.set_color(Color::by_index(92));
extbut.set_frame(FrameType::GleamRoundUpBox);
extbut.set_down_frame(FrameType::GleamUpFrame);
extbut.set_label_font(Font::by_index(13));
extbut.set_label_color(Color::by_index(39));
let mut extin = input::Input::new(70,100,200,25,"Enter Extension");
extin.set_selection_color(Color::by_index(132));
extin.set_frame(FrameType::RoundUpBox);
extin.set_label_font(Font::by_index(13));
extin.set_label_color(Color::by_index(92));
extin.set_text_color(Color::by_index(39));
    let mut slash = "/*";
    let mut extension = inval.replace("file://","").to_owned() + &slash.to_owned();
    let mut extfiles:Vec<String>=Vec::new();
    let mut bargroupext = group::Scroll::new(0, 0, 700, 500, None);
   
    //let mut filesizeoutput = output::Output::new(560, y0, 95, 25, None);
bargroup.set_color(Color::by_index(39));
bargroupext.hide();
extwin.end();

extbut.set_callback(move |extbut| {
    let mut extensions2 = &(extension.to_owned() + &extin.value().to_owned());
    
	GetExtensions(&extensions2, &mut extfiles);
let mut ind:usize = 0;
let mut ys = 0;
let mut extensionSize: f64 = 0.0;
let mut extension_percentage = 0.0;

    for file in &extfiles {
        let mut x = get_size(&file).unwrap() as f64;
        let mut fileoutput = output::Output::new(0, ys, 350, 25, None);
        let mut filesizeoutput = output::Output::new(360, ys, 105, 25, None);
        fileoutput.set_selection_color(Color::by_index(132));
        fileoutput.set_frame(FrameType::RoundUpBox);
        fileoutput.set_label_font(Font::by_index(13));
        fileoutput.set_label_color(Color::by_index(92));
        
        filesizeoutput.set_selection_color(Color::by_index(132));
        filesizeoutput.set_frame(FrameType::RoundUpBox);
        filesizeoutput.set_label_font(Font::by_index(13));
        filesizeoutput.set_label_color(Color::by_index(92));
        let slashes = "/*";
    let mut shortext = extension.replace("*","");
    let mut shorter2 =file.replace((&shortext),"");
//println!("{}", extension) ;
let mut inkb = x/1000.00;
let mut add = inkb.to_string().to_owned() + " KB";
extensionSize = extensionSize + x;

fileoutput.set_value(&shorter2);
filesizeoutput.set_value(&add);
 bargroupext.add(&fileoutput);
 bargroupext.add(&filesizeoutput);
 ys = ys +25;
    }
    let mut fileoutput = output::Output::new(0, ys, 350, 25, None);
let mut filesizeoutput = output::Output::new(360, ys, 105, 25, None);
    extension_percentage = extensionSize/ tot * 100.0;
    fileoutput.set_value("Percentage Occoupied by File extension");
filesizeoutput.set_value(&extension_percentage.to_string());

fileoutput.set_color(Color::by_index(92));
fileoutput.set_selection_color(Color::by_index(92));
fileoutput.set_frame(FrameType::RoundUpBox);
fileoutput.set_label_font(Font::by_index(13));
fileoutput.set_label_color(Color::by_index(92));

filesizeoutput.set_color(Color::by_index(92));
filesizeoutput.set_selection_color(Color::by_index(92));
filesizeoutput.set_frame(FrameType::RoundUpBox);
filesizeoutput.set_label_font(Font::by_index(13));
filesizeoutput.set_label_color(Color::by_index(92));
bargroupext.add(&fileoutput);
 bargroupext.add(&filesizeoutput);
  extbut.hide();
  extin.hide();
  bargroupext.show();
  //let mut bargroupext = group::Scroll::new(0, 0, 700, 500, None);

     
       
       });
       
       extwin.make_resizable(true);
       extwin.end();
       extwin.show();
});

let mut createfold = ui2.create.clone();

let ui4 = myuifile::UserInterface4::create_window();
let mut createwin = ui4.createwin.clone();
let mut createbut = ui4.createbut.clone();
let mut createfold = ui4.createfold.clone();
let mut createfile = ui4.createfile.clone();
let mut createout = ui4.createout.clone();
let mut filval = foldin.value();
createwin.hide();
createwin.make_resizable(true);
createout.hide();
ui2.create.set_callback(move |_| {createwin.end();createwin.show();});
createbut.set_callback(move |_| {let mut shorter2 = &filval.replace("file://","");
let mut creation = shorter2.to_owned()+ "/"+ &createfold.value().to_owned();
let mut creation2 = shorter2.to_owned()+ "/"+ &createfile.value().to_owned();
if Path::new(&creation).exists() == true && createfold.changed()
{
    createout.show();
    createout.cut();
    createout.set_value("Directory Exists");
}
else{
   
    fs::create_dir_all(&creation);
    
    createout.show();
    createout.cut();
    createout.set_value("Directory Created");
}
if  Path::new(&creation2).exists() ==true && createfile.changed()
{

    createout.show();
    createout.set_value("Directory Exists");
}
else {
    let mut file = File::create(&creation2);
    if Path::new(&creation).exists() == true || Path::new(&creation2).exists() == true{

        createout.show();
        createout.set_value("Directory Created");}
}

});
let ui5 = myuifile::UserInterface5::delete_window();
let mut fileval = foldin.value();
let mut deletefold = ui2.delete.clone();
let mut deletewin = ui5.deletewin.clone();
let mut delete = ui5.delettefile.clone();
let mut deletebut = ui5.deletebut.clone();
let mut deleteout = ui5.deleteout.clone();

deletewin.make_resizable(true);
deletewin.hide();
deleteout.hide();
ui2.delete.set_callback(move |_| {deletewin.end();deletewin.show() });
deletebut.set_callback(move |_| {let mut shorter2 = &fileval.replace("file://","");
let mut creation = shorter2.to_owned()+ "/"+ &delete.value().to_owned();
println!("{}" , creation);
if Path::new(&creation).exists() == true
{
    println!("{}" , creation);
    fs::remove_dir_all(&creation); 
    deleteout.show();
    deleteout.clear_changed();
    deleteout.set_value("Directory Deleted");
}
else{
    deleteout.show();
    deleteout.clear_changed();
    deleteout.set_value("Could not Find Directory");
}
});

});





app.run().unwrap();
}

fn GetExtensions(extension: &String, extfiles : &mut Vec<String>) -> Result<(), Box<dyn Error>>  {
	let mut size: usize = 0;
    let mut y0 :i32 = 0;
    
//let mut extension: &String;
    println! ( "extension  {}", extension);
        println!("file name with ext: {}", &extension);
       
       if extension != ""{

	for entry in glob(&extension)? {
		
        extfiles.push( entry?.display().to_string());
        //println!("{}", extfiles);
      
    
		size = size + 1;
    }
    
}
    else if size == 0 {
    	println!("There are no files with provided extension...");
    }
    else {
        println!("There are no files with provided extension...");

    }

    Ok(())
}
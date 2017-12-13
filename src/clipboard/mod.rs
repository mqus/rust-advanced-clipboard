extern crate mime;
mod x11;
mod error;

use std::vec::Vec;
use mime::Mime;

pub trait ClipboardOwner{

}

pub trait Transferable{
    fn get_data_flavours(&self)->&[Mime];
    fn is_flavour_supported(&self,flavour: &Mime) ->bool{
        self.get_data_flavours().contains(flavour)
    }
    fn get_data(&self,flavour:&Mime)->&[u8];
}

pub struct Clipboard {
    name: String,
    owner: Option<Box<ClipboardOwner>>,
    content: Option<Box<Transferable>>,
    listeners: Vec<Box<Fn()>>
}

impl Clipboard{
    pub fn new(name:String) -> Clipboard{
        Clipboard {
            name,
            owner: None,
            content: None,
            listeners: Vec::new(),
        }

    }
    pub fn get_name(&self) -> &str{
        return self.name.as_ref()
    }

    pub fn set_contents(&mut self, content:Box<Transferable>, owner:Box<ClipboardOwner>) {
        self.content=Some(content);
        self.owner=Some(owner);
        //TODO lost_ownership
        self.fire_flavor_change();
    }

    fn fire_flavor_change(&self){
        for x in &self.listeners{
            let y=x.as_ref();
            (*y)();
        }
    }

    fn get_contents(&self)->Option<&Transferable>{
        match &self.content{
            &None => None,
            &Some(ref x) => Some(x.as_ref())
        }
    }

    fn available_data_flavours(&self) -> &[Mime]{
        match &self.content{
            &None => &[],
            &Some(ref x) => x.as_ref().get_data_flavours()
        }
    }

    fn is_data_flavour_available(&self,flavour:&Mime)->Result<bool,error::NoDataError>{
        match &self.content{
            &None => Err(error::NoDataError{}),
            &Some(ref x) => Ok(x.as_ref().is_flavour_supported(flavour))
        }
    }



}

extern crate mime;
mod x11;
mod error;

use std::vec::Vec;
use std::cell::RefCell;
use std::rc::{Weak,Rc};
use mime::Mime;

pub trait ClipboardOwner{
    fn lost_ownership(&mut self, source:&Clipboard, new_content:Option<&Transferable>);
}

pub trait Transferable{
    fn get_data_flavours(&self)->&[Mime];
    fn is_flavour_supported(&self,flavour: &Mime) ->bool{
        self.get_data_flavours().contains(flavour)
    }
    fn get_data(&self,flavour:&Mime)->&[u8];
}

pub trait ClipboardObserver{
    fn clipboard_changed(&mut self);
}

pub struct Clipboard {
    name: String,
    owner: Option<Weak<RefCell<ClipboardOwner>>>,
    content: Option<Box<Transferable>>,
    listeners: Vec<Weak<RefCell<ClipboardObserver>>>
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

    pub fn set_contents(&mut self, content:Box<Transferable>, owner:&Rc<RefCell<ClipboardOwner>>) {
        if let Some(ref old_owner)=self.owner{
            if let Some(old_owner) = old_owner.upgrade(){
                if Rc::ptr_eq(&old_owner,owner) {
                    old_owner.borrow_mut().lost_ownership(&self, Some(content.as_ref()))
                }
            }
        }
        self.content=Some(content);
        self.owner=Some(Rc::downgrade(owner));
        self.fire_flavor_change();
    }

    fn fire_flavor_change(&self){
        for x in &self.listeners{
            if let Some(y)=x.upgrade(){
                y.borrow_mut().clipboard_changed();
            }
        }
    }

    pub fn get_contents(&self)->Option<&Transferable>{
        match &self.content{
            &None => None,
            &Some(ref x) => Some(x.as_ref())
        }
    }

    pub fn available_data_flavours(&self) -> &[Mime]{
        match &self.content{
            &None => &[],
            &Some(ref x) => x.as_ref().get_data_flavours()
        }
    }

    pub fn is_data_flavour_available(&self,flavour:&Mime)->Result<bool,error::NoDataError>{
        match &self.content{
            &None => Err(error::NoDataError{}),
            &Some(ref x) => Ok(x.as_ref().is_flavour_supported(flavour))
        }
    }

    pub fn get_data(&self,flavour:&Mime)->Result<&[u8],error::NoDataError>{
        match &self.content{
            &None => Err(error::NoDataError{}),
            &Some(ref x) => Ok(x.as_ref().get_data(flavour))
        }
    }

    pub fn register_change_callback(&mut self, callback:&Rc<RefCell<ClipboardObserver>>) {
        self.listeners.push(Rc::downgrade(callback));
    }

    pub fn unregister_change_callback(&mut self,callback:&Rc<RefCell<ClipboardObserver>>){
        self.listeners.retain(|x| {
            match x.upgrade(){
                None => false,//delete all 'dangling' references
                Some(y) => !Rc::ptr_eq(&y,callback)//delete only the given callback
            }
        });
    }


}

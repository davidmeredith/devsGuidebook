pub struct DraftReport {
    content: String,
}

pub struct UnpublishedReport {
    content: String,
}

pub struct PublishedReport {
    content: String,
}

impl DraftReport {
    pub fn new(content: &str) -> DraftReport {
        DraftReport {
            content: content.to_string(),
        }
    }

    pub fn edit(mut self, new_content: &str) -> DraftReport {
        self.content = new_content.to_string();
        // must return the given DraftReport instance (self) to continue to
        // work with it after calling this method as it is now owned by this block.
        self
    }

    pub fn submit(self) -> UnpublishedReport {
        // Calling this function means 'self/DraftReport' instance is moved into this
        // function - it cannot be used again. It is lost/gone as
        // it is not returned, a new UnpublishedReport is returned instead.
        UnpublishedReport {
            content: self.content,
        }
    }
}

impl UnpublishedReport {
    pub fn reject(self) -> DraftReport {
        DraftReport {
            content: self.content,
        }
    }

    pub fn publish(self) -> PublishedReport {
        PublishedReport {
            content: self.content,
        }
    }
}

fn main() {
    let str_a: String = String::from("a");
    let mut refer: &String = &str_a;
    if refer == "a" {
        let mut str_b: String = String::from("b");
        str_b.push_str(refer);
        // line below errors: 'str_b' does not live long enough
        //refer = &str_b;
    }
    println!("{refer}");

    let draft = DraftReport::new("Initial draft"); //immutable
                                                   // Edit the draft (repeatedly if needed). Need to shadow/overwrite draft
    let draft = draft.edit("Edited draft");

    // ✅Submit, but then change your mind and try to edit again - you can't, it won't compile,
    // once you submit, you've consumed/moved the draft variable so we can't re-use it.
    //let unpublished = draft.submit();
    //let draft = draft.edit("Oops I forgot something"); // ERROR!

    // ✅Uncommenting the next line means the subsequent line won't compile as 'draft' var is
    // moved into PublishedReport{} so we can't keep using 'draft'.
    // We would need to make a clone/copy of the draft.content first.
    //let directly_published = PublishedReport {
    //    content: draft.content,
    //}; // draft is moved into PublishedReport here

    let draft = draft.edit("Final draft");
    let unpublished = draft.submit();

    // We can reject it back to draft
    let draft_again = unpublished.reject();
    let unpublished = draft_again.submit();

    // Finally publish it
    let published = unpublished.publish();

    // Try to edit the published report
    // let edited_published = published.edit("Oops"); // ERROR! No such method

    // The compiler prevents publishing mistakes
    println!("Published: {}", published.content); // Published: Final draft
}

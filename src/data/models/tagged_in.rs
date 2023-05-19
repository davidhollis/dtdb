use diesel::prelude::*;
use identifier_prefix::identifier_prefix;

use crate::data::{identifiers::Identifier, schema::tagged_in};

use super::{media::Media, any::Any, person::Person, show::Show};

#[derive(Debug, Identifiable, Queryable, Selectable, Associations)]
#[diesel(belongs_to(Media))]
#[diesel(table_name = tagged_in)]
#[identifier_prefix(tag)]
pub struct TaggedIn {
    pub id: Identifier<TaggedIn>,
    pub media_id: Identifier<Media>,
    pub subject_id: Identifier<Any>,
}

impl TaggedIn {
    pub fn tag_person(person_id: Identifier<Person>, media_id: Identifier<Media>) -> NewTag {
        NewTag {
            id: Identifier::generate(),
            media_id,
            subject_id: person_id.to_any()
        }
    }

    pub fn tag_show(show_id: Identifier<Show>, media_id: Identifier<Media>) -> NewTag {
        NewTag {
            id: Identifier::generate(),
            media_id,
            subject_id: show_id.to_any()
        }
    }
}

#[derive(Identifiable, Insertable)]
#[diesel(table_name = tagged_in)]
pub struct NewTag {
    id: Identifier<TaggedIn>,
    media_id: Identifier<Media>,
    subject_id: Identifier<Any>,
}
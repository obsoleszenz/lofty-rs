use super::item::ItemKey;
use super::picture::Picture;
use crate::{PictureType, TagType};

/// Represents a tag item (key/value)
pub struct TagItem {
	item_key: ItemKey,
	item_value: ItemValue,
}

impl TagItem {
	/// Create a new [`TagItem`]
	///
	/// NOTES:
	///
	/// * This will check for validity based on the [`TagType`].
	/// * If the [`ItemKey`] does not map to a key in the target format, `None` will be returned.
	/// * It is pointless to do this if you plan on using [`Tag::insert_item`], as it does validity checks itself.
	pub fn new_checked(
		tag_type: &TagType,
		item_key: ItemKey,
		item_value: ItemValue,
	) -> Option<Self> {
		item_key.map_key(tag_type).is_some().then(|| Self {
			item_key,
			item_value,
		})
	}

	/// Create a new [`TagItem`]
	pub fn new(item_key: ItemKey, item_value: ItemValue) -> Self {
		Self {
			item_key,
			item_value,
		}
	}

	/// Returns a reference to the [`ItemKey`]
	pub fn key(&self) -> &ItemKey {
		&self.item_key
	}

	/// Returns a reference to the [`ItemValue`]
	pub fn value(&self) -> &ItemValue {
		&self.item_value
	}

	pub(crate) fn re_map(self, tag_type: &TagType) -> Option<Self> {
		self.item_key.map_key(tag_type).is_some().then(|| self)
	}
}

/// Represents a tag item's value
///
/// NOTE: The [Locator][ItemValue::Locator] and [Binary][ItemValue::Binary] variants are only applicable to APE tags.
/// Attempting to write either to another file/tag type will **not** error, they will just be ignored.
pub enum ItemValue {
	/// Any UTF-8 encoded text
	Text(String),
	/// **(APE ONLY)** Any UTF-8 encoded locator of external information
	Locator(String),
	/// **(APE ONLY)** Binary information, most likely a picture
	Binary(Vec<u8>),
}

/// Represents a parsed tag
///
/// NOTE: Items and pictures are separated
pub struct Tag {
	tag_type: TagType,
	pictures: Vec<Picture>,
	items: Vec<TagItem>,
}

impl Tag {
	/// Returns the [`TagType`]
	pub fn tag_type(&self) -> &TagType {
		&self.tag_type
	}

	/// Returns the number of [`Picture`]s
	pub fn picture_count(&self) -> u32 {
		self.pictures.len() as u32
	}

	/// Returns the number of [`TagItem`]s
	pub fn item_count(&self) -> u32 {
		self.items.len() as u32
	}
}

impl Tag {
	/// Returns the stored [`Picture`]s as a slice
	pub fn pictures(&self) -> &[Picture] {
		&*self.pictures
	}

	/// Pushes a [`Picture`] to the tag
	pub fn push_picture(&mut self, picture: Picture) {
		self.pictures.push(picture)
	}

	/// Removes all [`Picture`]s of a [`PictureType`]
	pub fn remove_picture_type(&mut self, picture_type: PictureType) {
		self.pictures
			.iter()
			.position(|p| p.pic_type == picture_type)
			.map(|pos| self.pictures.remove(pos));
	}

	/// Removes any matching [`Picture`]
	pub fn remove_picture(&mut self, picture: &Picture) {
		self.pictures.retain(|p| p != picture)
	}
}

impl Tag {
	/// Returns the stored [`TagItem`]s as a slice
	pub fn items(&self) -> &[TagItem] {
		&*self.items
	}

	/// Returns a reference to a [`TagItem`] matching an [`ItemKey`]
	pub fn get_item_ref(&self, item_key: &ItemKey) -> Option<&TagItem> {
		self.items.iter().find(|i| &i.item_key == item_key)
	}

	/// Insert a [`TagItem`], replacing any existing one of the same type
	///
	/// # Returns
	///
	/// This returns a bool if the item was successfully inserted/replaced.
	///
	/// `false` is only returned if the [`TagItem`]'s key couldn't be remapped to the target [`TagType`]
	pub fn insert_item(&mut self, item: TagItem) -> bool {
		if let Some(item) = item.re_map(&self.tag_type) {
			match self.items.iter_mut().find(|i| i.item_key == item.item_key) {
				None => self.items.push(item),
				Some(i) => *i = item,
			};

			return true;
		}

		false
	}
}

#[derive(Clone, Debug, PartialEq)]
pub enum TagType {
	#[cfg(feature = "format-ape")]
	/// Common file extensions: `.ape`
	Ape,
	#[cfg(feature = "format-id3")]
	/// Represents multiple formats, see [`Id3Format`](Id3Format) for extensions.
	Id3v2,
	#[cfg(feature = "format-mp4")]
	/// Common file extensions: `.mp4, .m4a, .m4p, .m4b, .m4r, .m4v`
	Mp4,
	#[cfg(feature = "format-opus")]
	/// Metadata stored in an Opus comment header
	/// Common file extensions: `.opus`
	Opus,
	#[cfg(feature = "format-vorbis")]
	/// Metadata stored in an OGG Vorbis file
	/// Common file extensions: `.ogg`
	Vorbis,
	#[cfg(feature = "format-flac")]
	/// Metadata stored in FLAC VORBISCOMMENT/PICTURE blocks
	/// Common file extensions: `.flac`
	Flac,
	#[cfg(feature = "format-riff")]
	/// Metadata stored in a RIFF INFO chunk
	/// Common file extensions: `.wav, .wave`
	RiffInfo,
	#[cfg(feature = "format-aiff")]
	/// Metadata stored in AIFF text chunks
	/// Common file extensions: `.aiff, .aif`
	AiffText,
}

use crate::{
    media::models::{
        DiscoveredLaunch, DiscoveredMedia, DiscoveredMediaMetadata, MediaLaunchType, MediaType,
    },
    storefronts::models::StorefrontId,
};

#[derive(Default)]
pub struct DiscoveredMediaBuilder {
    storefront: Option<StorefrontId>,
    external_id: Option<String>,
    media_type: Option<MediaType>,
    title: Option<String>,
    metadata: Option<DiscoveredMediaMetadata>,
    launch: Option<DiscoveredLaunch>,
}

impl DiscoveredMediaBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn storefront(mut self, storefront: StorefrontId) -> Self {
        self.storefront = Some(storefront);
        self
    }

    pub fn external_id(mut self, external_id: impl Into<String>) -> Self {
        self.external_id = Some(external_id.into());
        self
    }

    pub fn media_type(mut self, media_type: MediaType) -> Self {
        self.media_type = Some(media_type);
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn metadata(mut self, metadata: DiscoveredMediaMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn launch(mut self, launch: DiscoveredLaunch) -> Self {
        self.launch = Some(launch);
        self
    }

    pub fn build(self) -> DiscoveredMedia {
        DiscoveredMedia {
            storefront: self.storefront.unwrap_or(StorefrontId::Steam),
            external_id: self.external_id.unwrap_or_else(|| "1234".into()),
            media_type: self.media_type.unwrap_or(MediaType::Game),
            title: self.title.unwrap_or_else(|| "Test Game".into()),
            metadata: self.metadata.unwrap_or(DiscoveredMediaMetadata {
                total_playtime: None,
            }),
            launch: self.launch,
        }
    }
}

#[derive(Default)]
pub struct DiscoveredLaunchBuilder {
    name: Option<String>,
    launch_type: Option<MediaLaunchType>,
    program: Option<String>,
    arguments: Vec<String>,
    working_directory: Option<String>,
}

impl DiscoveredLaunchBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn launch_type(mut self, launch_type: MediaLaunchType) -> Self {
        self.launch_type = Some(launch_type);
        self
    }

    pub fn program(mut self, program: impl Into<String>) -> Self {
        self.program = Some(program.into());
        self
    }

    pub fn arguments<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.arguments = args.into_iter().map(Into::into).collect();
        self
    }

    pub fn working_directory(mut self, dir: impl Into<String>) -> Self {
        self.working_directory = Some(dir.into());
        self
    }

    pub fn build(self) -> DiscoveredLaunch {
        DiscoveredLaunch {
            name: self.name.unwrap_or_else(|| "Play".into()),
            launch_type: self.launch_type.unwrap_or(MediaLaunchType::Storefront),
            program: self.program.unwrap_or_else(|| "steam".into()),
            arguments: self.arguments,
            working_directory: self.working_directory,
        }
    }
}

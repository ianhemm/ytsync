
/**
 * A set of raw global parameters that are not meant to be used directly.
 * Instead, the api that requires a specific parameter will use this to get the specific key it needs to properly form a request.
 *
 * These are different from the Authorization methods(OAuth and ApiAuth).
 * the Authorization methods, while they have their own key
 * that is used like a parameter, also change what api methods are 
 * available to the user, the api parameters themselves
 * depend on what command is being run at the time,
 * and therefore dont need the same type restrictions
 * besides being hidden as an implementation detail.
 *
 * This is just to prevent duplicating code(much).
 *
 * Add values as needed, anything with its own unique
 * key needs to have an enum here.
 */
pub enum ApiParameter {

}

impl ApiParameter {
    pub fn key(param: ApiParameter) -> &str {
		match param {
		}
    }
}

pub enum ApiCommand {
	// Activity,
	// Caption,
	// ChannelBanner,
	// Channel,
	// ChannelSection,
	// Comment,
	// CommentThread,
	// I18nLanguage,
	// I18nRegion,
	// Member,
	// MembershipsLevel,
	PlaylistItems,
	// Playlist,
	// Search,
	// Subscription,
	// Thumbnail,
	// VideoAbuseReportReason,
	// VideoCategory,
	// Video,
	// Watermark,
}

/// The 'Material Design Icons' is the official icon set from Google.
/// The icons are designed under the material design guidelines.
/// The official repository is no longer maintained from Google.
/// A successosr project is hosted at:
/// https://github.com/material-icons/material-icons
/// This repository includes only icons as icon font

/*
 * Howto evaluate the glyph codes
 */
// emacs: to insert a glyph, use insert-char (bind:: C-x 8 Ret)
// eg: 'Edit'-Glyph ""
// - dec code point => (insert-char 57848 1)
// - hex code_point => C-x 8 Ret #xe1f8 Ret
// - oct code_point => C-x 8 Ret #o377 Ret
//
// Hex string to dezimal
// (string-to-number "ex1f8" 16) => 57848
// Hex string to dezimal
// (string-to-number "160770" 8) => 57848
// Dezimal string to hex
// (format "%X" 57848) ;;        => "e1f8"

/*
 * CSS Definiton:
 */
// CSS-url:     https://github.com/material-icons/material-icons-font/blob/master/css/baseline.css
// font-family: "Material Icons";
// font-weight: normal;
// font-style:  normal;
// font-format: ttf
// font-src:    MaterialIcons.ttf

//pub const MD_: &str = "";

// icons (hex-code accending)
pub const MD_360: &str = "";
pub const MD_3D: &str = "";
pub const MD_4K: &str = "";
pub const MD_AC_UNIT: &str = "";
pub const MD_ACCESS_ALARM: &str = "";
pub const MD_ACCESS_ALARMS: &str = "";
pub const MD_ACCESSIBILITY: &str = "";
pub const MD_ACCESSIBILITY_NEW: &str = "";
pub const MD_ACCESSIBLE: &str = "";
pub const MD_ACCESSIBLE_FORWARD: &str = "";
pub const MD_ACCOUNT_BALANCE: &str = "";
pub const MD_ACCOUNT_BALANCE_WALLET: &str = "";
pub const MD_ACCOUNT_BOX: &str = "";
pub const MD_ACCOUNT_CIRCLE: &str = "";
pub const MD_ACCOUNT_TREE: &str = "";
pub const MD_ADB: &str = "";
pub const MD_ADD: &str = "";
pub const MD_ADD_A_PHOTO: &str = "";
pub const MD_ADD_ALARM: &str = "";
pub const MD_ADD_ALERT: &str = "";
pub const MD_ADD_BOX: &str = "";
pub const MD_ADD_CIRCLE: &str = "";
pub const MD_ADD_CIRCLE_OUTLINE: &str = "";
pub const MD_ADD_PHOTO_ALTERNATE: &str = "";
pub const MD_ADD_SHPPING_CART: &str = "";
pub const MD_ADD_TO_HOME_SCREEN: &str = "";
pub const MD_ADD_TO_PHOTOS: &str = "";
pub const MD_ADD_TO_QUEUE: &str = "";
pub const MD_AIRLINE_SEAT_FLAT_ANGLED: &str = "";
pub const MD_AIRLINE_SEAT_INDIVIDUAL_SUITE: &str = "";
pub const MD_AIRLANEMODE_INACTIVE: &str = "";
pub const MD_AIRLAY: &str = "";
pub const MD_AIRPORT_SHUTTLE: &str = "";
pub const MD_ALARM: &str = "";
pub const MD_ALARM_ADD: &str = "";
pub const MD_ALARM_OFF: &str = "";
pub const MD_ALARM_ON: &str = "";
pub const MD_ALBUM: &str = "";
pub const MD_ALL_INBOX: &str = "";
pub const MD_ALL_INCLUSIVE: &str = "";
pub const MD_ALL_OUT: &str = "";
pub const MD_ALTERNATE_EMAIL: &str = "";
pub const MD_AM_STORIES: &str = "";
pub const MD_ANDROID: &str = "";
pub const MD_ANNOUNCEMENT: &str = "";
pub const MD_APARTMEMT: &str = "";
pub const MD_APPS: &str = "";
pub const MD_ARCHIVE: &str = "";
pub const MD_ARROW_BACK: &str = "";
pub const MD_ARROW_BACK_IOS: &str = "";
pub const MD_ARROW_DOWNWARD: &str = "";
pub const MD_ARROW_DROP_DOWN: &str = "";
pub const MD_ARROW_DROP_DOWN_CIRCLE: &str = "";
pub const MD_ARROW_DROP_UP: &str = "";
pub const MD_ARROW_FORWARD: &str = "";
pub const MD_ARROW_FORWARD_IOS: &str = "";
pub const MD_ARROW_LEFT: &str = "";
pub const MD_ARROW_RIGHT: &str = "";
pub const MD_ARROW_RIGHT_ALT: &str = "";
pub const MD_ARROW_UPWARD: &str = "";
pub const MD_ART_ACK: &str = "";
pub const MD_ASPECT_RATIO: &str = "";
pub const MD_ASSESSMENT: &str = "";
pub const MD_ASSIGNMENT: &str = "";
pub const MD_ASSIGNMENT_IND: &str = "";
pub const MD_ASSIGNMENT_LATE: &str = "";
pub const MD_ASSIGNMENT_RETURN: &str = "";
pub const MD_ASSIGNMENT_RETURNED: &str = "";
pub const MD_ASSIGNMENT_TURNED_IN: &str = "";
pub const MD_ASSISTANT: &str = "";
pub const MD_ASSISTANT_PHOTO: &str = "";
pub const MD_ATM: &str = "";
pub const MD_ATTACH_FILE: &str = "";
pub const MD_ATTACH_MONEY: &str = "";
pub const MD_ATTACHMENT: &str = "";
pub const MD_AUDIOTRACK: &str = "";
pub const MD_AUTORENEW: &str = "";
pub const MD_AV_TIMER: &str = "";
pub const MD_BACKSPACE: &str = "";
pub const MD_BACKUP: &str = "";
pub const MD_BALLOT: &str = "";
pub const MD_BAR_CHART: &str = "";
pub const MD_BARCODE: &str = "";
pub const MD_BATHTUB: &str = "";
pub const MD_BATTERY_20_AFTER: &str = "";
pub const MD_BATTERY_20_BEFORE: &str = "";
pub const MD_BATTERY_30_AFTER: &str = "";
pub const MD_BATTERY_30_BEFORE: &str = "";
pub const MD_BATTERY_50_AFTER: &str = "";
pub const MD_BATTERY_50_BEFORE: &str = "";
pub const MD_BATTERY_60_AFTER: &str = "";
pub const MD_BATTERY_60_BEFORE: &str = "";
pub const MD_BATTERY_80_AFTER: &str = "";
pub const MD_BATTERY_80_BEFORE: &str = "";
pub const MD_BATTERY_90_AFTER: &str = "";
pub const MD_BATTERY_90_BEFORE: &str = "";
pub const MD_BATTERY_ALERT: &str = "";
pub const MD_BATTERY_CHARGING_20_AFTER: &str = "";
pub const MD_BATTERY_CHARGING_20_BEFORE: &str = "";
pub const MD_BATTERY_CHARGING_30_AFTER: &str = "";
pub const MD_BATTERY_CHARGING_30_BEFORE: &str = "";
pub const MD_BATTERY_CHARGING_50_AFTER: &str = "";
pub const MD_BATTERY_CHARGING_50_BEFORE: &str = "";
pub const MD_BATTERY_CHARGING_60_AFTER: &str = "";
pub const MD_BATTERY_CHARGING_60_BEFORE: &str = "";
pub const MD_BATTERY_CHARGING_80_AFTER: &str = "";
pub const MD_BATTERY_CHARGING_80_BEFORE: &str = "";
pub const MD_BATTERY_CHARGING_90_AFTER: &str = "";
pub const MD_BATTERY_CHARGING_90_BEFORE: &str = "";
pub const MD_BATTERY_CHARGING_FULL: &str = "";
pub const MD_BATTERY: &str = "";
pub const MD_BATTERY_STD: &str = "";
pub const MD_BATTERY_UNKNOWN: &str = "";
pub const MD_BEACH_ACCESS: &str = "";
pub const MD_BEENHERE: &str = "";
pub const MD_BLOCK: &str = "";
pub const MD_BLUETOOTH: &str = "";
pub const MD_BLUETOOTH_AUDIO: &str = "";
pub const MD_BLUETOOTH_CONNECTED: &str = "";
pub const MD_BLUETOOTH_DISABLED: &str = "";
pub const MD_BLUETOOTH_SEARCHING: &str = "";
pub const MD_BLUR_CIRCULAR: &str = "";
pub const MD_BLUR_LINEAR: &str = "";
pub const MD_BLUR_OFF: &str ="";
pub const MD_BLUR_ON: &str = "";
pub const MD_BOOK: &str = "";
pub const MD_BOOKMARK: &str = "";
pub const MD_BOOKMARK_BORDER: &str = "";
pub const MD_BOOKMARKS: &str = "";
pub const MD_BORDER_ALL: &str = "";
pub const MD_BORDER_BOTTOM: &str = "";
pub const MD_BORDER_CLEAR_AFTER: &str = "";
pub const MD_BORDER_COLOR_AFTER: &str = "";
pub const MD_BORDER_COLOR_BEFORE: &str = "";
pub const MD_BORDER_HORIZONTAL: &str = "";
pub const MD_BORDER_INNER: &str = "";
pub const MD_BORDER_LEFT: &str = "";
pub const MD_BORDER_OUTER: &str = "";
pub const MD_BORDER_RIGHT: &str = "";
pub const MD_BORDER_STYLE: &str = "";
pub const MD_BORDER_STYLE_ALT: &str = "";
pub const MD_BORDER_VERTICAL: &str = "";
pub const MD_WATERMARK: &str = "";
pub const MD_BRIGHTNESS_2: &str = "";
pub const MD_BRIGHTNESS_3: &str = "";
pub const MD_BRIGHTNESS_4: &str = "";
pub const MD_BRIGHTNESS_5: &str = "";
pub const MD_BRIGHTNESS_6: &str = "";
pub const MD_BRIGHTNESS_7: &str = "";
pub const MD_BRIGHTNESS_AUTO: &str = "";
pub const MD_BRIGHTNESS_HIGH: &str = "";
pub const MD_BRIGHTNESS_LOW: &str = "";
pub const MD_BRIGHTNESS_MIDIUM: &str = "";
pub const MD_BROKEN_IMAGE: &str = "";
pub const MD_BRUSH_AFTER: &str = "";
pub const MD_BUBBLE_CHART: &str = "";
pub const MD_BUG_REPORT: &str = "";
pub const MD_BUILD: &str = "";
pub const MD_BURST_MODE: &str = "";
pub const MD_BUSINESS: &str = "";
pub const MD_BUSINESS_CENTER: &str = "";
pub const MD_CACHED: &str = "";
pub const MD_CAKE: &str = "";
pub const MD_CALENDAR_ODAY: &str = "";
pub const MD_CALENDAR_VIEW_DAY: &str = "";
pub const MD_CALL: &str = "";
pub const MD_CALL_END: &str = "";
pub const MD_CALL_MADE: &str = "";
pub const MD_CALL_MERGE: &str = "";
pub const MD_CALL_MISSED: &str = "";
pub const MD_CALL_MISSED_OUTGOING: &str = "";
pub const MD_CALL_RECEIVED: &str = "";
pub const MD_CALL_SPLIT: &str = "";
pub const MD_CALL_TO_ACTION: &str = "";
pub const MD_CAMERA: &str = "";
pub const MD_CAMERA_ALT: &str = "";
pub const MD_CAMERA_ENHANCE: &str = "";
pub const MD_CAMERA_FRONT: &str = "";
pub const MD_CAMERA_REAR: &str = "";
pub const MD_CAMERA_ROLL: &str = "";
pub const MD_CANCEL: &str = "";
pub const MD_CANCEL_PRESENTATION: &str = "";
pub const MD_CANCEL_SCHEDULE_SEND: &str = "";
pub const MD_CARD_GIFTCARD: &str = "";
pub const MD_CARD_MEMBERSHIP: &str = "";
pub const MD_CARD_TRAVEL: &str = "";
pub const MD_CASOMP: &str = "";
pub const MD_CAST: &str = "";
pub const MD_CAST_CONNECTED: &str = "";
pub const MD_CAST_FOR_EDUCATION: &str = "";
pub const MD_CATEGORY: &str = "";
pub const MD_CELL_WIFI_AFTER: &str = "";
pub const MD_CELL_WIFI_BEFORE: &str = "";
pub const MD_CENTER_FCUS_STRONG: &str = "";
pub const MD_CENTER_FOCUS_WEAK: &str = "";
pub const MD_CHANGE_HISTORY: &str = "";
pub const MD_CHAT: &str = "";
pub const MD_CHAT_BUBBLE: &str = "";
pub const MD_CHAT_BUBBLE_OUTLINE: &str = "";
pub const MD_CHECK: &str = "";
pub const MD_CHECK_BOX: &str = "";
pub const MD_CHECK_BOX_OUTLINE_BLANK: &str = "";
pub const MD_CHECK_CIRCLE: &str = "";
pub const MD_CHECK_CIRCLE_OUTLINE: &str = "";
pub const MD_CHEVRON_LEFT: &str = "";
pub const MD_CHEVRON_RIGHT: &str = "";
pub const MD_CHILD_CARE: &str = "";
pub const MD_CHILD_FRIENDLY: &str = "";
pub const MD_CHROME_READER_MODE: &str = "";
pub const MD_CLASS: &str = "";
pub const MD_CLEAR: &str = "";
pub const MD_CLEAR_ALL: &str = "";
pub const MD_CLOSE: &str = "";
pub const MD_CLOSED_CAPTION: &str = "";
pub const MD_CLOUD: &str = "";
pub const MD_CLOUD_CIRCLE: &str = "";
pub const MD_CLOUD_DONE: &str = "";
pub const MD_CLOUD_DOWNLOAD: &str = "";
pub const MD_CLOUD_OFF: &str = "";
pub const MD_CLOUD_QUEUE: &str = "";
pub const MD_CLOUD_UPLOAD: &str = "";
pub const MD_CODE: &str = "";
pub const MD_COLLECTIONS: &str = "";
pub const MD_COLLECTIONS_BOOKMARK: &str = "";
pub const MD_COLOR_LENS: &str = "";
pub const MD_COLORIZE: &str = "";
pub const MD_COMMENT: &str = "";
pub const MD_COMMUTE: &str = "";
pub const MD_COMPARE: &str = "";
pub const MD_COMPARE_ARROWS: &str = "";
pub const MD_COMPASS_CALIBRATION: &str = "";
pub const MD_COMPUTER: &str = "";
pub const MD_CONFIRMATION: &str = "";
pub const MD_CONTACT_MAIL: &str = "";
pub const MD_CONTACT_PHONE: &str = "";
pub const MD_CONTACT_SUPPORT: &str = "";
pub const MD_CONTACTLESS: &str = "";
pub const MD_CONTACTS: &str = "";
pub const MD_CONTENT_COPY: &str = "";
pub const MD_CONTENT_CUT: &str = "";
pub const MD_CONTENT_PASTE: &str = "";
pub const MD_CONTROL_CAMERA: &str = "";
pub const MD_CONTROL_POINT: &str = "";
pub const MD_CONTROL_POINT_DUPLICATE: &str = "";
pub const MD_COPYRIGHT: &str = "";
pub const MD_CREATE: &str = "";
pub const MD_CREATE_NEW_FOLDER: &str = "";
pub const MD_CREDIT_CARD: &str = "";
pub const MD_CROP: &str = "";
pub const MD_CROP_16_9: &str = "";
pub const MD_CROP_3_2: &str = "";
pub const MD_CROP_5_4: &str = "";
pub const MD_CROP_7_5: &str = "";
pub const MD_CROP_DIN: &str = "";
pub const MD_CROP_FREE: &str = "";
pub const MD_CROP_LANDSCAPE: &str = "";
pub const MD_CROP_ORIGINAL: &str = "";
pub const MD_CROP_PORTRAIT: &str = "";
pub const MD_CROP_ROTATE: &str = "";
pub const MD_CROP_SQUARE: &str = "";
pub const MD_DELETE: &str = "";
pub const MD_EDIT: &str = "";
pub const MD_DASHBOARD: &str = "";
pub const MD_DATA_USAGE: &str = "";
pub const MD_DATE_RANGE: &str = "";
pub const MD_DECK: &str = "";
pub const MD_DEHAZE: &str = "";
pub const MD_DEPARTURE_BOARD: &str = "";
pub const MD_DESCRIPTION: &str = "";
pub const MD_DESKTOP_ACCESS_DISABLED: &str = "";
pub const MD_DESKTOP_MAC: &str = "";
pub const MD_DESKTOP_WINDOWS: &str = "";
pub const MD_DETAILS: &str = "";
pub const MD_DEVELOPER_BOARD: &str = "";
pub const MD_DEVELOPER_MODE: &str = "";
pub const MD_DEVICE_HUB: &str = "";
pub const MD_DEVICE_UNKNOWN: &str = "";
pub const MD_DEVICES: &str = "";
pub const MD_DEVICES_OTHER: &str = "";
pub const MD_DIALER_SIP: &str = "";
pub const MD_DIALPAD: &str = "";
pub const MD_DIRECTIONS: &str = "";
pub const MD_DDIRECTIONS_BIKE: &str = "";
pub const MD_DIRECTIONS_BOAT: &str = "";
pub const MD_DIRECTIONS_BUS: &str = "";
pub const MD_DIRECTIONS_CAR: &str = "";
pub const MD_DIRECTIONS_RAILWAY: &str = "";
pub const MD_DIRECTIONS_RUN: &str = "";
pub const MD_DIRECTIONS_SUBWAY: &str = "";
pub const MD_DIRECTIONS_TRANSIT: &str = "";
pub const MD_DIRECTIONS_WLAK: &str = "";
pub const MD_DISC_FULL: &str = "";
pub const MD_DIVIDE: &str = "";
pub const MD_DNS: &str = "";
pub const MD_DO_NOT_DISTURB: &str = "";
pub const MD_DO_NOT_DISTURB_ALT: &str = "";
pub const MD_DO_NOT_DISTURB_OFF: &str = "";
pub const MD_DOCK: &str = "";
pub const MD_DOMAIN: &str = "";
pub const MD_DOMAIN_DISABLED: &str = "";
pub const MD_DONE: &str = "";
pub const MD_DONE_ALL: &str = "";
pub const MD_DONE_OUTLINE: &str = "";
pub const MD_DONUT_LARGE: &str = "";
pub const MD_DONUT_SMALL: &str = "";
pub const MD_DOUBLE_ARROW: &str = "";
pub const MD_DRAFTS: &str = "";
pub const MD_DRAG_HANDLE: &str = "";
pub const MD_DRAG_INDICATOR: &str = "";
pub const MD_DRIVE_ETA: &str = "";
pub const MD_DUO: &str = "";
pub const MD_DVR: &str = "";
pub const MD_DYNAMIC_FEED: &str = "";
pub const MD_ECO: &str = "";
pub const MD_EDIT_ATTRIBUTES: &str = "";
pub const MD_EDIT_LOCATION: &str = "";
pub const MD_EJECT: &str = "";
pub const MD_EMAIL: &str = "";
pub const MD_EMOJI_EMOTIONS: &str = "";
pub const MD_EMOJI_EVENTS: &str = "";
pub const MD_EMOJI_FLAGS: &str = "";
pub const MD_EMOJI_FOOD_BEVERAGE: &str = "";
pub const MD_EMOJI_NATURE: &str = "";
pub const MD_EMOJI_OBJECTS: &str = "";
pub const MD_EMOJI_PEOPLE: &str = "";
pub const MD_EMOJI_SYMBOLS: &str = "";
pub const MD_EMOJI_TRANSPORTATION: &str = "";
pub const MD_ENHANCED_ENCRYPTION: &str = "";
pub const MD_EQUALIZER: &str = "";
pub const MD_EQUALS: &str = "";
pub const MD_ERROR: &str = "";
pub const MD_ERROR_OUTLINE: &str = "";
pub const MD_EURO: &str = "";
pub const MD_EURO_SYMBOL: &str = "";
pub const MD_EV_STATION: &str = "";
pub const MD_EVENT: &str = "";
pub const MD_EVENT_AVAILABLE: &str = "";
pub const MD_EVENT_BUSY: &str = "";
pub const MD_EVENT_NOTE: &str = "";
pub const MD_EVENT_SEAT: &str = "";
pub const MD_EXIT_TO_APP: &str = "";
pub const MD_EXPAND_LESS: &str = "";
pub const MD_EXPAND_MORE: &str = "";
pub const MD_EXLICIT: &str = "";
pub const MD_EXPLORE: &str = "";
pub const MD_EXPLORE_OFF: &str = "";
pub const MD_EXPOSURE: &str = "";
pub const MD_EXPOSURE_NEG_1: &str = "";
pub const MD_EXPOSURE_NEG_2: &str = "";
pub const MD_EXPOSURE_PLUS_1: &str = "";
pub const MD_EXPOSURE_PLUS_2: &str = "";
pub const MD_EXPOSURE_ZERO: &str = "";
pub const MD_EXTENSION: &str = "";
pub const MD_DELETE_FOREVER: &str = "";
pub const MD_DELETE_OUTLINE: &str = "";
pub const MD_DELETE_SWEEP: &str = "";
pub const MD_KEYBOARD_ARROW_DOWN: &str = "";
pub const MD_KEYBOARD_ARROW_LEFT: &str = "";
pub const MD_KEYBOARD_ARROW_RIGHT: &str = "";
pub const MD_KEYBOARD_ARROW_UP: &str = "";
pub const MD_KEYBOARD_BACKSPACE: &str = "";
pub const MD_KEYBOARD_CAPSLOCK: &str = "";
pub const MD_KEYBOARD_HIDE: &str = "";
pub const MD_KEYBOARD_ARROW_RETURN: &str = "";
pub const MD_KEYBOARD_TAB: &str = "";
pub const MD_MENU: &str = "";
pub const MD_MINUS: &str = "";
pub const MD_PLUS: &str = "";
pub const MD_PLUS_MINUS: &str = "";
pub const MD_PLUS_ONE: &str = "";
pub const MD_POWER: &str = "";
pub const MD_POWER_INPUT: &str = "";
pub const MD_POWER_OFF: &str = "";
pub const MD_POWER_SETTINGS_NEW: &str = "";
pub const MD_REMOVE: &str = "";
pub const MD_REMOVE_CIRCLE: &str = "";
pub const MD_SAVE: &str = "";

/*
 *  WIP: this unicodes needs your update!
 */

// last edited: dec= 57752 -> e0198



// .md-face:after {
//   content: '\e238';
// }

// .md-fast_forward:after {
//   content: '\e23a';
// }

// .md-fast_rewind:after {
//   content: '\e23c';
// }

// .md-fastfood:after {
//   content: '\e23e';
// }

// .md-favorite:after {
//   content: '\e240';
// }

// .md-favorite_border:after {
//   content: '\e242';
// }

// .md-featured_play_list:after {
//   content: '\e243';
// }

// .md-featured_video:after {
//   content: '\e245';
// }

// .md-feedback:after {
//   content: '\e247';
// }

// .md-fiber_dvr:after {
//   content: '\e249';
// }

// .md-fiber_manual_record:after {
//   content: '\e24b';
// }

// .md-fiber_new:after {
//   content: '\e24d';
// }

// .md-fiber_pin:after {
//   content: '\e24f';
// }

// .md-fiber_smart_record:after {
//   content: '\e251';
// }

// .md-file_copy:after {
//   content: '\e253';
// }

// .md-file_upload:after {
//   content: '\e255';
// }

// .md-filter:after {
//   content: '\e257';
// }

// .md-filter_1:after {
//   content: '\e259';
// }

// .md-filter_2:after {
//   content: '\e25b';
// }

// .md-filter_3:after {
//   content: '\e25d';
// }

// .md-filter_4:after {
//   content: '\e25f';
// }

// .md-filter_5:after {
//   content: '\e261';
// }

// .md-filter_6:after {
//   content: '\e263';
// }

// .md-filter_7:after {
//   content: '\e265';
// }

// .md-filter_8:after {
//   content: '\e267';
// }

// .md-filter_9:after {
//   content: '\e269';
// }

// .md-filter_9_plus:after {
//   content: '\e26b';
// }

// .md-filter_b_and_w:after {
//   content: '\e26d';
// }

// .md-filter_center_focus:after {
//   content: '\e26f';
// }

// .md-filter_drama:after {
//   content: '\e270';
// }

// .md-filter_frames:after {
//   content: '\e272';
// }

// .md-filter_hdr:after {
//   content: '\e274';
// }

// .md-filter_list:after {
//   content: '\e276';
// }

// .md-filter_none:after {
//   content: '\e277';
// }

// .md-filter_tilt_shift:after {
//   content: '\e279';
// }

// .md-filter_vintage:after {
//   content: '\e27a';
// }

// .md-find_in_page:after {
//   content: '\e27c';
// }

// .md-find_replace:after {
//   content: '\e27e';
// }

// .md-fingerprint:after {
//   content: '\e27f';
// }

// .md-fireplace:after {
//   content: '\e280';
// }

// .md-first_page:after {
//   content: '\e282';
// }

// .md-fitness_center:after {
//   content: '\e283';
// }

// .md-flag:after {
//   content: '\e284';
// }

// .md-flare:after {
//   content: '\e286';
// }

// .md-flash_auto:after {
//   content: '\e287';
// }

// .md-flash_off:after {
//   content: '\e288';
// }

// .md-flash_on:after {
//   content: '\e289';
// }

// .md-flight:after {
//   content: '\e28a';
// }

// .md-flight_land:after {
//   content: '\e28b';
// }

// .md-flight_takeoff:after {
//   content: '\e28c';
// }

// .md-flip:after {
//   content: '\e28d';
// }

// .md-flip_camera_android:after {
//   content: '\e28e';
// }

// .md-flip_camera_ios:after {
//   content: '\e290';
// }

// .md-flip_to_back:after {
//   content: '\e292';
// }

// .md-flip_to_front:after {
//   content: '\e293';
// }

// .md-folder:after {
//   content: '\e294';
// }

// .md-folder_open:after {
//   content: '\e296';
// }

// .md-folder_shared:after {
//   content: '\e298';
// }

// .md-folder_special:after {
//   content: '\e29a';
// }

// .md-font_download:after {
//   content: '\e29c';
// }

// .md-format_align_center:after {
//   content: '\e29e';
// }

// .md-format_align_justify:after {
//   content: '\e29f';
// }

// .md-format_align_left:after {
//   content: '\e2a0';
// }

// .md-format_align_right:after {
//   content: '\e2a1';
// }

// .md-format_bold:after {
//   content: '\e2a2';
// }

// .md-format_clear:after {
//   content: '\e2a3';
// }

// .md-format_color_fill:after {
//   content: '\e2a4';
// }

// .md-format_color_fill:before {
//   content: '\e2a5';
// }

// .md-format_color_reset:after {
//   content: '\e2a6';
// }

// .md-format_color_text:after {
//   content: '\e2a8';
// }

// .md-format_color_text:before {
//   content: '\e2a9';
// }

// .md-format_indent_decrease:after {
//   content: '\e2aa';
// }

// .md-format_indent_increase:after {
//   content: '\e2ab';
// }

// .md-format_italic:after {
//   content: '\e2ac';
// }

// .md-format_line_spacing:after {
//   content: '\e2ad';
// }

// .md-format_list_bulleted:after {
//   content: '\e2ae';
// }

// .md-format_list_numbered:after {
//   content: '\e2af';
// }

// .md-format_list_numbered_rtl:after {
//   content: '\e2b0';
// }

// .md-format_paint:after {
//   content: '\e2b1';
// }

// .md-format_quote:after {
//   content: '\e2b3';
// }

// .md-format_shapes:after {
//   content: '\e2b5';
// }

// .md-format_size:after {
//   content: '\e2b7';
// }

// .md-format_strikethrough:after {
//   content: '\e2b8';
// }

// .md-format_textdirection_l_to_r:after {
//   content: '\e2b9';
// }

// .md-format_textdirection_r_to_l:after {
//   content: '\e2bb';
// }

// .md-format_underlined:after {
//   content: '\e2bd';
// }

// .md-forum:after {
//   content: '\e2be';
// }

// .md-forward:after {
//   content: '\e2c0';
// }

// .md-forward_10:after {
//   content: '\e2c2';
// }

// .md-forward_30:after {
//   content: '\e2c3';
// }

// .md-forward_5:after {
//   content: '\e2c4';
// }

// .md-free_breakfast:after {
//   content: '\e2c5';
// }

// .md-fullscreen:after {
//   content: '\e2c7';
// }

// .md-fullscreen_exit:after {
//   content: '\e2c8';
// }

// .md-functions:after {
//   content: '\e2c9';
// }

// .md-g_translate:after {
//   content: '\e2ca';
// }

// .md-gamepad:after {
//   content: '\e2cb';
// }

// .md-games:after {
//   content: '\e2cd';
// }

// .md-gavel:after {
//   content: '\e2cf';
// }

// .md-gesture:after {
//   content: '\e2d0';
// }

// .md-get_app:after {
//   content: '\e2d1';
// }

// .md-gif:after {
//   content: '\e2d3';
// }

// .md-golf_course:after {
//   content: '\e2d5';
// }

// .md-gps_fixed:after {
//   content: '\e2d7';
// }

// .md-gps_not_fixed:after {
//   content: '\e2d9';
// }

// .md-gps_off:after {
//   content: '\e2da';
// }

// .md-grade:after {
//   content: '\e2db';
// }

// .md-gradient:after {
//   content: '\e2dd';
// }

// .md-grain:after {
//   content: '\e2de';
// }

// .md-graphic_eq:after {
//   content: '\e2df';
// }

// .md-greater_than:after {
//   content: '\e2e0';
// }

// .md-greater_than_equal:after {
//   content: '\e2e1';
// }

// .md-grid_off:after {
//   content: '\e2e2';
// }

// .md-grid_on:after {
//   content: '\e2e4';
// }

// .md-group:after {
//   content: '\e2e6';
// }

// .md-group_add:after {
//   content: '\e2e8';
// }

// .md-group_work:after {
//   content: '\e2ea';
// }

// .md-hd:after {
//   content: '\e2ec';
// }

// .md-hdr_off:after {
//   content: '\e2ee';
// }

// .md-hdr_on:after {
//   content: '\e2ef';
// }

// .md-hdr_strong:after {
//   content: '\e2f0';
// }

// .md-hdr_weak:after {
//   content: '\e2f2';
// }

// .md-headset:after {
//   content: '\e2f4';
// }

// .md-headset_mic:after {
//   content: '\e2f6';
// }

// .md-healing:after {
//   content: '\e2f8';
// }

// .md-hearing:after {
//   content: '\e2fa';
// }

// .md-height:after {
//   content: '\e2fb';
// }

// .md-help:after {
//   content: '\e2fc';
// }

// .md-help_outline:after {
//   content: '\e2fe';
// }

// .md-high_quality:after {
//   content: '\e2ff';
// }

// .md-highlight:after {
//   content: '\e301';
// }

// .md-highlight_off:after {
//   content: '\e303';
// }

// .md-history:after {
//   content: '\e305';
// }

// .md-home:after {
//   content: '\e306';
// }

// .md-home_work:after {
//   content: '\e308';
// }

// .md-horizontal_split:after {
//   content: '\e30a';
// }

// .md-hot_tub:after {
//   content: '\e30c';
// }

// .md-hotel:after {
//   content: '\e30d';
// }

// .md-hourglass_empty:after {
//   content: '\e30f';
// }

// .md-hourglass_full:after {
//   content: '\e310';
// }

// .md-house:after {
//   content: '\e312';
// }

// .md-how_to_reg:after {
//   content: '\e314';
// }

// .md-how_to_vote:after {
//   content: '\e316';
// }

// .md-http:after {
//   content: '\e318';
// }

// .md-https:after {
//   content: '\e319';
// }

// .md-image:after {
//   content: '\e31b';
// }

// .md-image_aspect_ratio:after {
//   content: '\e31d';
// }

// .md-image_search:after {
//   content: '\e31f';
// }

// .md-import_contacts:after {
//   content: '\e321';
// }

// .md-import_export:after {
//   content: '\e323';
// }

// .md-important_devices:after {
//   content: '\e324';
// }

// .md-inbox:after {
//   content: '\e326';
// }

// .md-indeterminate_check_box:after {
//   content: '\e328';
// }

// .md-info:after {
//   content: '\e32a';
// }

// .md-input:after {
//   content: '\e32c';
// }

// .md-insert_chart:after {
//   content: '\e32d';
// }

// .md-insert_chart_outlined:after {
//   content: '\e32f';
// }

// .md-insert_comment:after {
//   content: '\e330';
// }

// .md-insert_drive_file:after {
//   content: '\e332';
// }

// .md-insert_emoticon:after {
//   content: '\e334';
// }

// .md-insert_invitation:after {
//   content: '\e336';
// }

// .md-insert_link:after {
//   content: '\e338';
// }

// .md-insert_photo:after {
//   content: '\e339';
// }

// .md-invert_colors:after {
//   content: '\e33b';
// }

// .md-invert_colors_off:after {
//   content: '\e33d';
// }

// .md-iso:after {
//   content: '\e33f';
// }

// .md-keyboard:after {
//   content: '\e341';
// }

// .md-keyboard_arrow_down:after {
//   content: '\e343';
// }

// .md-keyboard_voice:after {
//   content: '\e34d';
// }

// .md-king_bed:after {
//   content: '\e34f';
// }

// .md-kitchen:after {
//   content: '\e351';
// }

// .md-label:after {
//   content: '\e353';
// }

// .md-label_important:after {
//   content: '\e355';
// }

// .md-label_off:after {
//   content: '\e357';
// }

// .md-landscape:after {
//   content: '\e359';
// }

// .md-language:after {
//   content: '\e35b';
// }

// .md-laptop:after {
//   content: '\e35d';
// }

// .md-laptop_chromebook:after {
//   content: '\e35f';
// }

// .md-laptop_mac:after {
//   content: '\e361';
// }

// .md-laptop_windows:after {
//   content: '\e363';
// }

// .md-last_page:after {
//   content: '\e365';
// }

// .md-launch:after {
//   content: '\e366';
// }

// .md-layers:after {
//   content: '\e367';
// }

// .md-layers_clear:after {
//   content: '\e369';
// }

// .md-leak_add:after {
//   content: '\e36b';
// }

// .md-leak_remove:after {
//   content: '\e36c';
// }

// .md-lens:after {
//   content: '\e36d';
// }

// .md-less_than:after {
//   content: '\e36f';
// }

// .md-less_than_equal:after {
//   content: '\e370';
// }

// .md-library_add:after {
//   content: '\e371';
// }

// .md-library_books:after {
//   content: '\e373';
// }

// .md-library_music:after {
//   content: '\e375';
// }

// .md-lightbulb:after {
//   content: '\e377';
// }

// .md-line_style:after {
//   content: '\e379';
// }

// .md-line_weight:after {
//   content: '\e37a';
// }

// .md-linear_scale:after {
//   content: '\e37b';
// }

// .md-link:after {
//   content: '\e37c';
// }

// .md-link_off:after {
//   content: '\e37e';
// }

// .md-linked_camera:after {
//   content: '\e37f';
// }

// .md-list:after {
//   content: '\e381';
// }

// .md-list_alt:after {
//   content: '\e382';
// }

// .md-live_help:after {
//   content: '\e384';
// }

// .md-live_tv:after {
//   content: '\e386';
// }

// .md-local_activity:after {
//   content: '\e388';
// }

// .md-local_airport:after {
//   content: '\e38a';
// }

// .md-local_atm:after {
//   content: '\e38b';
// }

// .md-local_bar:after {
//   content: '\e38d';
// }

// .md-local_cafe:after {
//   content: '\e38f';
// }

// .md-local_car_wash:after {
//   content: '\e391';
// }

// .md-local_convenience_store:after {
//   content: '\e393';
// }

// .md-local_dining:after {
//   content: '\e395';
// }

// .md-local_drink:after {
//   content: '\e396';
// }

// .md-local_florist:after {
//   content: '\e398';
// }

// .md-local_gas_station:after {
//   content: '\e39a';
// }

// .md-local_grocery_store:after {
//   content: '\e39c';
// }

// .md-local_hospital:after {
//   content: '\e39e';
// }

// .md-local_hotel:after {
//   content: '\e3a0';
// }

// .md-local_laundry_service:after {
//   content: '\e3a2';
// }

// .md-local_library:after {
//   content: '\e3a4';
// }

// .md-local_mall:after {
//   content: '\e3a6';
// }

// .md-local_movies:after {
//   content: '\e3a8';
// }

// .md-local_offer:after {
//   content: '\e3aa';
// }

// .md-local_parking:after {
//   content: '\e3ac';
// }

// .md-local_pharmacy:after {
//   content: '\e3ad';
// }

// .md-local_phone:after {
//   content: '\e3af';
// }

// .md-local_pizza:after {
//   content: '\e3b1';
// }

// .md-local_play:after {
//   content: '\e3b3';
// }

// .md-local_post_office:after {
//   content: '\e3b5';
// }

// .md-local_printshop:after {
//   content: '\e3b7';
// }

// .md-local_see:after {
//   content: '\e3b9';
// }

// .md-local_shipping:after {
//   content: '\e3bb';
// }

// .md-local_taxi:after {
//   content: '\e3bd';
// }

// .md-location_city:after {
//   content: '\e3bf';
// }

// .md-location_disabled:after {
//   content: '\e3c0';
// }

// .md-location_off:after {
//   content: '\e3c1';
// }

// .md-location_on:after {
//   content: '\e3c2';
// }

// .md-location_searching:after {
//   content: '\e3c4';
// }

// .md-lock:after {
//   content: '\e3c5';
// }

// .md-lock_open:after {
//   content: '\e3c7';
// }

// .md-log_in:after {
//   content: '\e3c9';
// }

// .md-log_out:after {
//   content: '\e3ca';
// }

// .md-looks:after {
//   content: '\e3cb';
// }

// .md-looks_3:after {
//   content: '\e3cc';
// }

// .md-looks_4:after {
//   content: '\e3ce';
// }

// .md-looks_5:after {
//   content: '\e3d0';
// }

// .md-looks_6:after {
//   content: '\e3d2';
// }

// .md-looks_one:after {
//   content: '\e3d4';
// }

// .md-looks_two:after {
//   content: '\e3d6';
// }

// .md-loop:after {
//   content: '\e3d8';
// }

// .md-loupe:after {
//   content: '\e3d9';
// }

// .md-low_priority:after {
//   content: '\e3db';
// }

// .md-loyalty:after {
//   content: '\e3dc';
// }

// .md-mail:after {
//   content: '\e3de';
// }

// .md-mail_outline:after {
//   content: '\e3e0';
// }

// .md-map:after {
//   content: '\e3e1';
// }

// .md-markunread:after {
//   content: '\e3e3';
// }

// .md-markunread_mailbox:after {
//   content: '\e3e5';
// }

// .md-maximize:after {
//   content: '\e3e7';
// }

// .md-meeting_room:after {
//   content: '\e3e8';
// }

// .md-memory:after {
//   content: '\e3ea';
// }

// .md-menu:after {
//   content: '\e3ec';
// }

// .md-menu_book:after {
//   content: '\e3ed';
// }

// .md-menu_open:after {
//   content: '\e3ef';
// }

// .md-merge_type:after {
//   content: '\e3f0';
// }

// .md-message:after {
//   content: '\e3f1';
// }

// .md-mic:after {
//   content: '\e3f3';
// }

// .md-mic_none:after {
//   content: '\e3f5';
// }

// .md-mic_off:after {
//   content: '\e3f7';
// }

// .md-minimize:after {
//   content: '\e3f9';
// }

// .md-missed_video_call:after {
//   content: '\e3fb';
// }

// .md-mms:after {
//   content: '\e3fd';
// }

// .md-mobile_friendly:after {
//   content: '\e3ff';
// }

// .md-mobile_off:after {
//   content: '\e400';
// }

// .md-mobile_screen_share:after {
//   content: '\e401';
// }

// .md-mode_comment:after {
//   content: '\e403';
// }

// .md-monetization_on:after {
//   content: '\e405';
// }

// .md-money:after {
//   content: '\e407';
// }

// .md-money_off:after {
//   content: '\e409';
// }

// .md-monochrome_photos:after {
//   content: '\e40a';
// }

// .md-mood:after {
//   content: '\e40c';
// }

// .md-mood_bad:after {
//   content: '\e40e';
// }

// .md-more:after {
//   content: '\e410';
// }

// .md-more_horiz:after {
//   content: '\e412';
// }

// .md-more_vert:after {
//   content: '\e413';
// }

// .md-motorcycle:after {
//   content: '\e414';
// }

// .md-mouse:after {
//   content: '\e416';
// }

// .md-move_to_inbox:after {
//   content: '\e418';
// }

// .md-movie:after {
//   content: '\e41a';
// }

// .md-movie_creation:after {
//   content: '\e41c';
// }

// .md-movie_filter:after {
//   content: '\e41e';
// }

// .md-multiline_chart:after {
//   content: '\e420';
// }

// .md-museum:after {
//   content: '\e421';
// }

// .md-music_note:after {
//   content: '\e423';
// }

// .md-music_off:after {
//   content: '\e425';
// }

// .md-music_video:after {
//   content: '\e427';
// }

// .md-my_location:after {
//   content: '\e429';
// }

// .md-nature:after {
//   content: '\e42b';
// }

// .md-nature_people:after {
//   content: '\e42d';
// }

// .md-navigate_before:after {
//   content: '\e42f';
// }

// .md-navigate_next:after {
//   content: '\e430';
// }

// .md-navigation:after {
//   content: '\e431';
// }

// .md-near_me:after {
//   content: '\e433';
// }

// .md-network_cell:after {
//   content: '\e435';
// }

// .md-network_cell:before {
//   content: '\e436';
// }

// .md-network_check:after {
//   content: '\e437';
// }

// .md-network_locked:after {
//   content: '\e438';
// }

// .md-network_wifi:after {
//   content: '\e439';
// }

// .md-network_wifi:before {
//   content: '\e43a';
// }

// .md-new_releases:after {
//   content: '\e43b';
// }

// .md-next_week:after {
//   content: '\e43d';
// }

// .md-nfc:after {
//   content: '\e43f';
// }

// .md-nights_stay:after {
//   content: '\e440';
// }

// .md-no_encryption:after {
//   content: '\e442';
// }

// .md-no_meeting_room:after {
//   content: '\e444';
// }

// .md-no_sim:after {
//   content: '\e446';
// }

// .md-not_equal:after {
//   content: '\e448';
// }

// .md-not_interested:after {
//   content: '\e449';
// }

// .md-not_listed_location:after {
//   content: '\e44a';
// }

// .md-note:after {
//   content: '\e44c';
// }

// .md-note_add:after {
//   content: '\e44e';
// }

// .md-notes:after {
//   content: '\e450';
// }

// .md-notification_important:after {
//   content: '\e451';
// }

// .md-notifications:after {
//   content: '\e453';
// }

// .md-notifications_active:after {
//   content: '\e455';
// }

// .md-notifications_none:after {
//   content: '\e457';
// }

// .md-notifications_off:after {
//   content: '\e459';
// }

// .md-notifications_paused:after {
//   content: '\e45b';
// }

// .md-offline_bolt:after {
//   content: '\e45d';
// }

// .md-offline_pin:after {
//   content: '\e45f';
// }

// .md-ondemand_video:after {
//   content: '\e461';
// }

// .md-opacity:after {
//   content: '\e463';
// }

// .md-open_in_browser:after {
//   content: '\e465';
// }

// .md-open_in_new:after {
//   content: '\e466';
// }

// .md-open_with:after {
//   content: '\e467';
// }

// .md-outdoor_grill:after {
//   content: '\e468';
// }

// .md-outlined_flag:after {
//   content: '\e46a';
// }

// .md-pages:after {
//   content: '\e46b';
// }

// .md-pageview:after {
//   content: '\e46d';
// }

// .md-palette:after {
//   content: '\e46f';
// }

// .md-pan_tool:after {
//   content: '\e471';
// }

// .md-panorama:after {
//   content: '\e473';
// }

// .md-panorama_fish_eye:after {
//   content: '\e475';
// }

// .md-panorama_horizontal:after {
//   content: '\e477';
// }

// .md-panorama_vertical:after {
//   content: '\e479';
// }

// .md-panorama_wide_angle:after {
//   content: '\e47b';
// }

// .md-party_mode:after {
//   content: '\e47d';
// }

// .md-pause:after {
//   content: '\e47f';
// }

// .md-pause_circle_filled:after {
//   content: '\e480';
// }

// .md-pause_circle_outline:after {
//   content: '\e482';
// }

// .md-pause_presentation:after {
//   content: '\e483';
// }

// .md-payment:after {
//   content: '\e485';
// }

// .md-people:after {
//   content: '\e487';
// }

// .md-people_alt:after {
//   content: '\e489';
// }

// .md-people_outline:after {
//   content: '\e48b';
// }

// .md-percentage:after {
//   content: '\e48d';
// }

// .md-perm_camera_mic:after {
//   content: '\e48f';
// }

// .md-perm_contact_calendar:after {
//   content: '\e491';
// }

// .md-perm_data_setting:after {
//   content: '\e493';
// }

// .md-perm_device_information:after {
//   content: '\e494';
// }

// .md-perm_identity:after {
//   content: '\e496';
// }

// .md-perm_media:after {
//   content: '\e498';
// }

// .md-perm_phone_msg:after {
//   content: '\e49a';
// }

// .md-perm_scan_wifi:after {
//   content: '\e49c';
// }

// .md-person:after {
//   content: '\e49e';
// }

// .md-person_add:after {
//   content: '\e4a0';
// }

// .md-person_add_disabled:after {
//   content: '\e4a2';
// }

// .md-person_outline:after {
//   content: '\e4a4';
// }

// .md-person_pin:after {
//   content: '\e4a6';
// }

// .md-person_pin_circle:after {
//   content: '\e4a8';
// }

// .md-personal_video:after {
//   content: '\e4aa';
// }

// .md-pets:after {
//   content: '\e4ac';
// }

// .md-phone:after {
//   content: '\e4ad';
// }

// .md-phone_android:after {
//   content: '\e4af';
// }

// .md-phone_bluetooth_speaker:after {
//   content: '\e4b1';
// }

// .md-phone_callback:after {
//   content: '\e4b3';
// }

// .md-phone_disabled:after {
//   content: '\e4b5';
// }

// .md-phone_enabled:after {
//   content: '\e4b6';
// }

// .md-phone_forwarded:after {
//   content: '\e4b7';
// }

// .md-phone_in_talk:after {
//   content: '\e4b9';
// }

// .md-phone_iphone:after {
//   content: '\e4bb';
// }

// .md-phone_locked:after {
//   content: '\e4bd';
// }

// .md-phone_missed:after {
//   content: '\e4bf';
// }

// .md-phone_paused:after {
//   content: '\e4c1';
// }

// .md-phonelink:after {
//   content: '\e4c3';
// }

// .md-phonelink_erase:after {
//   content: '\e4c5';
// }

// .md-phonelink_lock:after {
//   content: '\e4c6';
// }

// .md-phonelink_off:after {
//   content: '\e4c7';
// }

// .md-phonelink_ring:after {
//   content: '\e4c9';
// }

// .md-phonelink_setup:after {
//   content: '\e4cb';
// }

// .md-photo:after {
//   content: '\e4cc';
// }

// .md-photo_album:after {
//   content: '\e4ce';
// }

// .md-photo_camera:after {
//   content: '\e4d0';
// }

// .md-photo_filter:after {
//   content: '\e4d2';
// }

// .md-photo_library:after {
//   content: '\e4d3';
// }

// .md-photo_size_select_actual:after {
//   content: '\e4d5';
// }

// .md-photo_size_select_large:after {
//   content: '\e4d7';
// }

// .md-photo_size_select_small:after {
//   content: '\e4d8';
// }

// .md-picture_as_pdf:after {
//   content: '\e4d9';
// }

// .md-picture_in_picture:after {
//   content: '\e4db';
// }

// .md-picture_in_picture_alt:after {
//   content: '\e4dd';
// }

// .md-pie_chart:after {
//   content: '\e4df';
// }

// .md-pin:after {
//   content: '\e4e1';
// }

// .md-pin_drop:after {
//   content: '\e4e3';
// }

// .md-pin_off:after {
//   content: '\e4e5';
// }

// .md-place:after {
//   content: '\e4e7';
// }

// .md-play_arrow:after {
//   content: '\e4e9';
// }

// .md-play_circle_filled:after {
//   content: '\e4eb';
// }

// .md-play_circle_filled_white:after {
//   content: '\e4ed';
// }

// .md-play_circle_outline:after {
//   content: '\e4ef';
// }

// .md-play_for_work:after {
//   content: '\e4f0';
// }

// .md-playlist_add:after {
//   content: '\e4f1';
// }

// .md-playlist_add_check:after {
//   content: '\e4f2';
// }

// .md-playlist_play:after {
//   content: '\e4f3';
// }

// .md-plus_one:after {
//   content: '\e4f7';
// }

// .md-policy:after {
//   content: '\e4f8';
// }

// .md-poll:after {
//   content: '\e4fa';
// }

// .md-polymer:after {
//   content: '\e4fc';
// }

// .md-pool:after {
//   content: '\e4fd';
// }

// .md-portable_wifi_off:after {
//   content: '\e4ff';
// }

// .md-portrait:after {
//   content: '\e500';
// }

// .md-post_add:after {
//   content: '\e502';
// }

// .md-power:after {
//   content: '\e503';
// }

// .md-power_input:after {
//   content: '\e505';
// }

// .md-power_off:after {
//   content: '\e506';
// }

// .md-power_settings_new:after {
//   content: '\e508';
// }

// .md-pregnant_woman:after {
//   content: '\e509';
// }

// .md-present_to_all:after {
//   content: '\e50a';
// }

// .md-print:after {
//   content: '\e50c';
// }

// .md-print_disabled:after {
//   content: '\e50e';
// }

// .md-priority_high:after {
//   content: '\e510';
// }

// .md-public:after {
//   content: '\e511';
// }

// .md-publish:after {
//   content: '\e513';
// }

// .md-qrcode:after {
//   content: '\e515';
// }

// .md-query_builder:after {
//   content: '\e517';
// }

// .md-question_answer:after {
//   content: '\e519';
// }

// .md-queue:after {
//   content: '\e51b';
// }

// .md-queue_music:after {
//   content: '\e51d';
// }

// .md-queue_play_next:after {
//   content: '\e51f';
// }

// .md-radio:after {
//   content: '\e520';
// }

// .md-radio_button_checked:after {
//   content: '\e522';
// }

// .md-radio_button_unchecked:after {
//   content: '\e523';
// }

// .md-rate_review:after {
//   content: '\e524';
// }

// .md-receipt:after {
//   content: '\e526';
// }

// .md-recent_actors:after {
//   content: '\e528';
// }

// .md-record_voice_over:after {
//   content: '\e52a';
// }

// .md-redeem:after {
//   content: '\e52c';
// }

// .md-redo:after {
//   content: '\e52e';
// }

// .md-refresh:after {
//   content: '\e52f';
// }

// .md-remove_circle_outline:after {
//   content: '\e533';
// }

// .md-remove_from_queue:after {
//   content: '\e534';
// }

// .md-remove_red_eye:after {
//   content: '\e536';
// }

// .md-remove_shopping_cart:after {
//   content: '\e538';
// }

// .md-reorder:after {
//   content: '\e53a';
// }

// .md-repeat:after {
//   content: '\e53b';
// }

// .md-repeat_one:after {
//   content: '\e53c';
// }

// .md-replay:after {
//   content: '\e53d';
// }

// .md-replay_10:after {
//   content: '\e53e';
// }

// .md-replay_30:after {
//   content: '\e53f';
// }

// .md-replay_5:after {
//   content: '\e540';
// }

// .md-reply:after {
//   content: '\e541';
// }

// .md-reply_all:after {
//   content: '\e542';
// }

// .md-report:after {
//   content: '\e543';
// }

// .md-report_off:after {
//   content: '\e545';
// }

// .md-report_problem:after {
//   content: '\e547';
// }

// .md-restaurant:after {
//   content: '\e549';
// }

// .md-restaurant_menu:after {
//   content: '\e54a';
// }

// .md-restore:after {
//   content: '\e54b';
// }

// .md-restore_from_trash:after {
//   content: '\e54c';
// }

// .md-restore_page:after {
//   content: '\e54e';
// }

// .md-ring_volume:after {
//   content: '\e550';
// }

// .md-rocket:after {
//   content: '\e552';
// }

// .md-room:after {
//   content: '\e554';
// }

// .md-room_service:after {
//   content: '\e556';
// }

// .md-rotate_90_degrees_ccw:after {
//   content: '\e558';
// }

// .md-rotate_left:after {
//   content: '\e55a';
// }

// .md-rotate_right:after {
//   content: '\e55b';
// }

// .md-rounded_corner:after {
//   content: '\e55c';
// }

// .md-router:after {
//   content: '\e55d';
// }

// .md-rowing:after {
//   content: '\e55f';
// }

// .md-rss_feed:after {
//   content: '\e560';
// }

// .md-rv_hookup:after {
//   content: '\e561';
// }

// .md-satellite:after {
//   content: '\e563';
// }

// .md-save_alt:after {
//   content: '\e567';
// }

// .md-scanner:after {
//   content: '\e568';
// }

// .md-scatter_plot:after {
//   content: '\e56a';
// }

// .md-schedule:after {
//   content: '\e56c';
// }

// .md-school:after {
//   content: '\e56e';
// }

// .md-score:after {
//   content: '\e570';
// }

// .md-screen_lock_landscape:after {
//   content: '\e572';
// }

// .md-screen_lock_portrait:after {
//   content: '\e574';
// }

// .md-screen_lock_rotation:after {
//   content: '\e576';
// }

// .md-screen_rotation:after {
//   content: '\e577';
// }

// .md-screen_share:after {
//   content: '\e579';
// }

// .md-sd_card:after {
//   content: '\e57b';
// }

// .md-sd_storage:after {
//   content: '\e57d';
// }

// .md-search:after {
//   content: '\e57f';
// }

// .md-security:after {
//   content: '\e580';
// }

// .md-select_all:after {
//   content: '\e582';
// }

// .md-send:after {
//   content: '\e583';
// }

// .md-sentiment_dissatisfied:after {
//   content: '\e585';
// }

// .md-sentiment_neutral:after {
//   content: '\e587';
// }

// .md-sentiment_satisfied:after {
//   content: '\e589';
// }

// .md-sentiment_satisfied_alt:after {
//   content: '\e58b';
// }

// .md-sentiment_slightly_dissatisfied:after {
//   content: '\e58d';
// }

// .md-sentiment_very_dissatisfied:after {
//   content: '\e58f';
// }

// .md-sentiment_very_satisfied:after {
//   content: '\e591';
// }

// .md-settings:after {
//   content: '\e593';
// }

// .md-settings_applications:after {
//   content: '\e595';
// }

// .md-settings_backup_restore:after {
//   content: '\e597';
// }

// .md-settings_bluetooth:after {
//   content: '\e598';
// }

// .md-settings_brightness:after {
//   content: '\e599';
// }

// .md-settings_cell:after {
//   content: '\e59b';
// }

// .md-settings_ethernet:after {
//   content: '\e59d';
// }

// .md-settings_input_antenna:after {
//   content: '\e59e';
// }

// .md-settings_input_component:after {
//   content: '\e59f';
// }

// .md-settings_input_composite:after {
//   content: '\e5a1';
// }

// .md-settings_input_hdmi:after {
//   content: '\e5a3';
// }

// .md-settings_input_svideo:after {
//   content: '\e5a5';
// }

// .md-settings_overscan:after {
//   content: '\e5a7';
// }

// .md-settings_phone:after {
//   content: '\e5a9';
// }

// .md-settings_power:after {
//   content: '\e5ab';
// }

// .md-settings_remote:after {
//   content: '\e5ac';
// }

// .md-settings_system_daydream:after {
//   content: '\e5ae';
// }

// .md-settings_voice:after {
//   content: '\e5b0';
// }

// .md-share:after {
//   content: '\e5b2';
// }

// .md-shop:after {
//   content: '\e5b4';
// }

// .md-shop_two:after {
//   content: '\e5b6';
// }

// .md-shopping_basket:after {
//   content: '\e5b8';
// }

// .md-shopping_cart:after {
//   content: '\e5ba';
// }

// .md-short_text:after {
//   content: '\e5bc';
// }

// .md-show_chart:after {
//   content: '\e5bd';
// }

// .md-shuffle:after {
//   content: '\e5be';
// }

// .md-shutter_speed:after {
//   content: '\e5bf';
// }

// .md-signal_cellular_0_bar:after {
//   content: '\e5c1';
// }

// .md-signal_cellular_0_bar:before {
//   content: '\e5c2';
// }

// .md-signal_cellular_1_bar:after {
//   content: '\e5c3';
// }

// .md-signal_cellular_1_bar:before {
//   content: '\e5c4';
// }

// .md-signal_cellular_2_bar:after {
//   content: '\e5c5';
// }

// .md-signal_cellular_2_bar:before {
//   content: '\e5c6';
// }

// .md-signal_cellular_3_bar:after {
//   content: '\e5c7';
// }

// .md-signal_cellular_3_bar:before {
//   content: '\e5c8';
// }

// .md-signal_cellular_4_bar:after {
//   content: '\e5c9';
// }

// .md-signal_cellular_alt:after {
//   content: '\e5ca';
// }

// .md-signal_cellular_connected_no_internet_0_bar:after {
//   content: '\e5cb';
// }

// .md-signal_cellular_connected_no_internet_0_bar:before {
//   content: '\e5cc';
// }

// .md-signal_cellular_connected_no_internet_1_bar:after {
//   content: '\e5cd';
// }

// .md-signal_cellular_connected_no_internet_1_bar:before {
//   content: '\e5ce';
// }

// .md-signal_cellular_connected_no_internet_2_bar:after {
//   content: '\e5cf';
// }

// .md-signal_cellular_connected_no_internet_2_bar:before {
//   content: '\e5d0';
// }

// .md-signal_cellular_connected_no_internet_3_bar:after {
//   content: '\e5d1';
// }

// .md-signal_cellular_connected_no_internet_3_bar:before {
//   content: '\e5d2';
// }

// .md-signal_cellular_connected_no_internet_4_bar:after {
//   content: '\e5d3';
// }

// .md-signal_cellular_no_sim:after {
//   content: '\e5d4';
// }

// .md-signal_cellular_null:after {
//   content: '\e5d6';
// }

// .md-signal_cellular_off:after {
//   content: '\e5d7';
// }

// .md-signal_wifi_0_bar:after {
//   content: '\e5d8';
// }

// .md-signal_wifi_0_bar:before {
//   content: '\e5d9';
// }

// .md-signal_wifi_1_bar:after {
//   content: '\e5da';
// }

// .md-signal_wifi_1_bar:before {
//   content: '\e5db';
// }

// .md-signal_wifi_1_bar_lock:after {
//   content: '\e5dc';
// }

// .md-signal_wifi_1_bar_lock:before {
//   content: '\e5dd';
// }

// .md-signal_wifi_2_bar:after {
//   content: '\e5de';
// }

// .md-signal_wifi_2_bar:before {
//   content: '\e5df';
// }

// .md-signal_wifi_2_bar_lock:after {
//   content: '\e5e0';
// }

// .md-signal_wifi_2_bar_lock:before {
//   content: '\e5e1';
// }

// .md-signal_wifi_3_bar:after {
//   content: '\e5e2';
// }

// .md-signal_wifi_3_bar:before {
//   content: '\e5e3';
// }

// .md-signal_wifi_3_bar_lock:after {
//   content: '\e5e4';
// }

// .md-signal_wifi_3_bar_lock:before {
//   content: '\e5e5';
// }

// .md-signal_wifi_4_bar:after {
//   content: '\e5e6';
// }

// .md-signal_wifi_4_bar_lock:after {
//   content: '\e5e7';
// }

// .md-signal_wifi_off:after {
//   content: '\e5e8';
// }

// .md-sim_card:after {
//   content: '\e5e9';
// }

// .md-sim_card_alert:after {
//   content: '\e5eb';
// }

// .md-single_bed:after {
//   content: '\e5ed';
// }

// .md-skip_next:after {
//   content: '\e5ef';
// }

// .md-skip_previous:after {
//   content: '\e5f1';
// }

// .md-slideshow:after {
//   content: '\e5f3';
// }

// .md-slow_motion_video:after {
//   content: '\e5f5';
// }

// .md-smartphone:after {
//   content: '\e5f6';
// }

// .md-smoke_free:after {
//   content: '\e5f8';
// }

// .md-smoking_rooms:after {
//   content: '\e5f9';
// }

// .md-sms:after {
//   content: '\e5fb';
// }

// .md-sms_failed:after {
//   content: '\e5fd';
// }

// .md-snooze:after {
//   content: '\e5ff';
// }

// .md-sort:after {
//   content: '\e600';
// }

// .md-sort_by_alpha:after {
//   content: '\e601';
// }

// .md-spa:after {
//   content: '\e602';
// }

// .md-space_bar:after {
//   content: '\e604';
// }

// .md-speaker:after {
//   content: '\e605';
// }

// .md-speaker_group:after {
//   content: '\e607';
// }

// .md-speaker_notes:after {
//   content: '\e609';
// }

// .md-speaker_notes_off:after {
//   content: '\e60b';
// }

// .md-speaker_phone:after {
//   content: '\e60d';
// }

// .md-speed:after {
//   content: '\e60f';
// }

// .md-spellcheck:after {
//   content: '\e610';
// }

// .md-sports:after {
//   content: '\e611';
// }

// .md-sports_baseball:after {
//   content: '\e612';
// }

// .md-sports_basketball:after {
//   content: '\e614';
// }

// .md-sports_cricket:after {
//   content: '\e616';
// }

// .md-sports_esports:after {
//   content: '\e618';
// }

// .md-sports_football:after {
//   content: '\e61a';
// }

// .md-sports_golf:after {
//   content: '\e61c';
// }

// .md-sports_handball:after {
//   content: '\e61e';
// }

// .md-sports_hockey:after {
//   content: '\e61f';
// }

// .md-sports_kabaddi:after {
//   content: '\e620';
// }

// .md-sports_mma:after {
//   content: '\e621';
// }

// .md-sports_motorsports:after {
//   content: '\e623';
// }

// .md-sports_rugby:after {
//   content: '\e625';
// }

// .md-sports_soccer:after {
//   content: '\e627';
// }

// .md-sports_tennis:after {
//   content: '\e629';
// }

// .md-sports_volleyball:after {
//   content: '\e62a';
// }

// .md-square_foot:after {
//   content: '\e62c';
// }

// .md-star:after {
//   content: '\e62e';
// }

// .md-star_border:after {
//   content: '\e630';
// }

// .md-star_half:after {
//   content: '\e631';
// }

// .md-star_rate:after {
//   content: '\e632';
// }

// .md-stars:after {
//   content: '\e633';
// }

// .md-stay_current_landscape:after {
//   content: '\e635';
// }

// .md-stay_current_portrait:after {
//   content: '\e637';
// }

// .md-stay_primary_landscape:after {
//   content: '\e639';
// }

// .md-stay_primary_portrait:after {
//   content: '\e63b';
// }

// .md-stop:after {
//   content: '\e63d';
// }

// .md-stop_circle:after {
//   content: '\e63f';
// }

// .md-stop_screen_share:after {
//   content: '\e641';
// }

// .md-storage:after {
//   content: '\e643';
// }

// .md-store:after {
//   content: '\e644';
// }

// .md-store_mall_directory:after {
//   content: '\e646';
// }

// .md-storefront:after {
//   content: '\e648';
// }

// .md-straighten:after {
//   content: '\e64a';
// }

// .md-streetview:after {
//   content: '\e64c';
// }

// .md-strikethrough_s:after {
//   content: '\e64d';
// }

// .md-style:after {
//   content: '\e64e';
// }

// .md-subdirectory_arrow_left:after {
//   content: '\e650';
// }

// .md-subdirectory_arrow_right:after {
//   content: '\e651';
// }

// .md-subject:after {
//   content: '\e652';
// }

// .md-subscriptions:after {
//   content: '\e653';
// }

// .md-subtitles:after {
//   content: '\e655';
// }

// .md-subway:after {
//   content: '\e657';
// }

// .md-supervised_user_circle:after {
//   content: '\e659';
// }

// .md-supervisor_account:after {
//   content: '\e65b';
// }

// .md-surround_sound:after {
//   content: '\e65d';
// }

// .md-swap_calls:after {
//   content: '\e65f';
// }

// .md-swap_horiz:after {
//   content: '\e660';
// }

// .md-swap_horizontal_circle:after {
//   content: '\e661';
// }

// .md-swap_vert:after {
//   content: '\e663';
// }

// .md-swap_vertical_circle:after {
//   content: '\e664';
// }

// .md-switch_camera:after {
//   content: '\e666';
// }

// .md-switch_video:after {
//   content: '\e668';
// }

// .md-sync:after {
//   content: '\e66a';
// }

// .md-sync_alt:after {
//   content: '\e66b';
// }

// .md-sync_disabled:after {
//   content: '\e66c';
// }

// .md-sync_problem:after {
//   content: '\e66d';
// }

// .md-system_update:after {
//   content: '\e66e';
// }

// .md-system_update_alt:after {
//   content: '\e670';
// }

// .md-tab:after {
//   content: '\e671';
// }

// .md-tab_unselected:after {
//   content: '\e672';
// }

// .md-table_chart:after {
//   content: '\e673';
// }

// .md-tablet:after {
//   content: '\e675';
// }

// .md-tablet_android:after {
//   content: '\e677';
// }

// .md-tablet_mac:after {
//   content: '\e679';
// }

// .md-tag_faces:after {
//   content: '\e67b';
// }

// .md-tap_and_play:after {
//   content: '\e67d';
// }

// .md-terrain:after {
//   content: '\e67e';
// }

// .md-text_fields:after {
//   content: '\e680';
// }

// .md-text_format:after {
//   content: '\e681';
// }

// .md-text_rotate_up:after {
//   content: '\e682';
// }

// .md-text_rotate_vertical:after {
//   content: '\e683';
// }

// .md-text_rotation_angledown:after {
//   content: '\e684';
// }

// .md-text_rotation_angleup:after {
//   content: '\e685';
// }

// .md-text_rotation_down:after {
//   content: '\e686';
// }

// .md-text_rotation_none:after {
//   content: '\e687';
// }

// .md-textsms:after {
//   content: '\e688';
// }

// .md-texture:after {
//   content: '\e68a';
// }

// .md-theaters:after {
//   content: '\e68b';
// }

// .md-thumb_down:after {
//   content: '\e68d';
// }

// .md-thumb_down_alt:after {
//   content: '\e68f';
// }

// .md-thumb_up:after {
//   content: '\e691';
// }

// .md-thumb_up_alt:after {
//   content: '\e693';
// }

// .md-thumbs_up_down:after {
//   content: '\e695';
// }

// .md-time_to_leave:after {
//   content: '\e697';
// }

// .md-timelapse:after {
//   content: '\e699';
// }

// .md-timeline:after {
//   content: '\e69b';
// }

// .md-timer:after {
//   content: '\e69c';
// }

// .md-timer_10:after {
//   content: '\e69e';
// }

// .md-timer_3:after {
//   content: '\e69f';
// }

// .md-timer_off:after {
//   content: '\e6a0';
// }

// .md-title:after {
//   content: '\e6a2';
// }

// .md-toc:after {
//   content: '\e6a3';
// }

// .md-today:after {
//   content: '\e6a4';
// }

// .md-toggle_off:after {
//   content: '\e6a6';
// }

// .md-toggle_on:after {
//   content: '\e6a8';
// }

// .md-toll:after {
//   content: '\e6aa';
// }

// .md-tonality:after {
//   content: '\e6ac';
// }

// .md-touch_app:after {
//   content: '\e6ae';
// }

// .md-toys:after {
//   content: '\e6b0';
// }

// .md-track_changes:after {
//   content: '\e6b2';
// }

// .md-traffic:after {
//   content: '\e6b3';
// }

// .md-train:after {
//   content: '\e6b5';
// }

// .md-tram:after {
//   content: '\e6b7';
// }

// .md-transfer_within_a_station:after {
//   content: '\e6b9';
// }

// .md-transform:after {
//   content: '\e6ba';
// }

// .md-transit_enterexit:after {
//   content: '\e6bb';
// }

// .md-translate:after {
//   content: '\e6bc';
// }

// .md-trending_down:after {
//   content: '\e6bd';
// }

// .md-trending_flat:after {
//   content: '\e6be';
// }

// .md-trending_up:after {
//   content: '\e6bf';
// }

// .md-trip_origin:after {
//   content: '\e6c0';
// }

// .md-tune:after {
//   content: '\e6c1';
// }

// .md-turned_in:after {
//   content: '\e6c2';
// }

// .md-turned_in_not:after {
//   content: '\e6c4';
// }

// .md-tv:after {
//   content: '\e6c5';
// }

// .md-tv_off:after {
//   content: '\e6c7';
// }

// .md-unarchive:after {
//   content: '\e6c9';
// }

// .md-undo:after {
//   content: '\e6cb';
// }

// .md-unfold_less:after {
//   content: '\e6cc';
// }

// .md-unfold_more:after {
//   content: '\e6cd';
// }

// .md-unsubscribe:after {
//   content: '\e6ce';
// }

// .md-update:after {
//   content: '\e6d0';
// }

// .md-usb:after {
//   content: '\e6d1';
// }

// .md-verified_user:after {
//   content: '\e6d2';
// }

// .md-vertical_align_bottom:after {
//   content: '\e6d4';
// }

// .md-vertical_align_center:after {
//   content: '\e6d5';
// }

// .md-vertical_align_top:after {
//   content: '\e6d6';
// }

// .md-vertical_split:after {
//   content: '\e6d7';
// }

// .md-vibration:after {
//   content: '\e6d9';
// }

// .md-video_call:after {
//   content: '\e6db';
// }

// .md-video_label:after {
//   content: '\e6dd';
// }

// .md-video_library:after {
//   content: '\e6df';
// }

// .md-videocam:after {
//   content: '\e6e1';
// }

// .md-videocam_off:after {
//   content: '\e6e3';
// }

// .md-videogame_asset:after {
//   content: '\e6e5';
// }

// .md-view_agenda:after {
//   content: '\e6e7';
// }

// .md-view_array:after {
//   content: '\e6e9';
// }

// .md-view_carousel:after {
//   content: '\e6eb';

// .md-rv_hookup:after {
//   content: '\e561';
// }

// .md-satellite:after {
//   content: '\e563';
// }

// .md-save_alt:after {
//   content: '\e567';
// }

// .md-scanner:after {
//   content: '\e568';
// }

// .md-scatter_plot:after {
//   content: '\e56a';
// }

// .md-schedule:after {
//   content: '\e56c';
// }

// .md-school:after {
//   content: '\e56e';
// }

// .md-score:after {
//   content: '\e570';
// }

// .md-screen_lock_landscape:after {
//   content: '\e572';
// }

// .md-screen_lock_portrait:after {
//   content: '\e574';
// }

// .md-screen_lock_rotation:after {
//   content: '\e576';
// }

// .md-screen_rotation:after {
//   content: '\e577';
// }

// .md-screen_share:after {
//   content: '\e579';
// }

// .md-sd_card:after {
//   content: '\e57b';
// }

// .md-sd_storage:after {
//   content: '\e57d';
// }

// .md-search:after {
//   content: '\e57f';
// }

// .md-security:after {
//   content: '\e580';
// }

// .md-select_all:after {
//   content: '\e582';
// }

// .md-send:after {
//   content: '\e583';
// }

// .md-sentiment_dissatisfied:after {
//   content: '\e585';
// }

// .md-sentiment_neutral:after {
//   content: '\e587';
// }

// .md-sentiment_satisfied:after {
//   content: '\e589';
// }

// .md-sentiment_satisfied_alt:after {
//   content: '\e58b';
// }

// .md-sentiment_slightly_dissatisfied:after {
//   content: '\e58d';
// }

// .md-sentiment_very_dissatisfied:after {
//   content: '\e58f';
// }

// .md-sentiment_very_satisfied:after {
//   content: '\e591';
// }

// .md-settings:after {
//   content: '\e593';
// }

// .md-settings_applications:after {
//   content: '\e595';
// }

// .md-settings_backup_restore:after {
//   content: '\e597';
// }

// .md-settings_bluetooth:after {
//   content: '\e598';
// }

// .md-settings_brightness:after {
//   content: '\e599';
// }

// .md-settings_cell:after {
//   content: '\e59b';
// }

// .md-settings_ethernet:after {
//   content: '\e59d';
// }

// .md-settings_input_antenna:after {
//   content: '\e59e';
// }

// .md-settings_input_component:after {
//   content: '\e59f';
// }

// .md-settings_input_composite:after {
//   content: '\e5a1';
// }

// .md-settings_input_hdmi:after {
//   content: '\e5a3';
// }

// .md-settings_input_svideo:after {
//   content: '\e5a5';
// }

// .md-settings_overscan:after {
//   content: '\e5a7';
// }

// .md-settings_phone:after {
//   content: '\e5a9';
// }

// .md-settings_power:after {
//   content: '\e5ab';
// }

// .md-settings_remote:after {
//   content: '\e5ac';
// }

// .md-settings_system_daydream:after {
//   content: '\e5ae';
// }

// .md-settings_voice:after {
//   content: '\e5b0';
// }

// .md-share:after {
//   content: '\e5b2';
// }

// .md-shop:after {
//   content: '\e5b4';
// }

// .md-shop_two:after {
//   content: '\e5b6';
// }

// .md-shopping_basket:after {
//   content: '\e5b8';
// }

// .md-shopping_cart:after {
//   content: '\e5ba';
// }

// .md-short_text:after {
//   content: '\e5bc';
// }

// .md-show_chart:after {
//   content: '\e5bd';
// }

// .md-shuffle:after {
//   content: '\e5be';
// }

// .md-shutter_speed:after {
//   content: '\e5bf';
// }

// .md-signal_cellular_0_bar:after {
//   content: '\e5c1';
// }

// .md-signal_cellular_0_bar:before {
//   content: '\e5c2';
// }

// .md-signal_cellular_1_bar:after {
//   content: '\e5c3';
// }

// .md-signal_cellular_1_bar:before {
//   content: '\e5c4';
// }

// .md-signal_cellular_2_bar:after {
//   content: '\e5c5';
// }

// .md-signal_cellular_2_bar:before {
//   content: '\e5c6';
// }

// .md-signal_cellular_3_bar:after {
//   content: '\e5c7';
// }

// .md-signal_cellular_3_bar:before {
//   content: '\e5c8';
// }

// .md-signal_cellular_4_bar:after {
//   content: '\e5c9';
// }

// .md-signal_cellular_alt:after {
//   content: '\e5ca';
// }

// .md-signal_cellular_connected_no_internet_0_bar:after {
//   content: '\e5cb';
// }

// .md-signal_cellular_connected_no_internet_0_bar:before {
//   content: '\e5cc';
// }

// .md-signal_cellular_connected_no_internet_1_bar:after {
//   content: '\e5cd';
// }

// .md-signal_cellular_connected_no_internet_1_bar:before {
//   content: '\e5ce';
// }

// .md-signal_cellular_connected_no_internet_2_bar:after {
//   content: '\e5cf';
// }

// .md-signal_cellular_connected_no_internet_2_bar:before {
//   content: '\e5d0';
// }

// .md-signal_cellular_connected_no_internet_3_bar:after {
//   content: '\e5d1';
// }

// .md-signal_cellular_connected_no_internet_3_bar:before {
//   content: '\e5d2';
// }

// .md-signal_cellular_connected_no_internet_4_bar:after {
//   content: '\e5d3';
// }

// .md-signal_cellular_no_sim:after {
//   content: '\e5d4';
// }

// .md-signal_cellular_null:after {
//   content: '\e5d6';
// }

// .md-signal_cellular_off:after {
//   content: '\e5d7';
// }

// .md-signal_wifi_0_bar:after {
//   content: '\e5d8';
// }

// .md-signal_wifi_0_bar:before {
//   content: '\e5d9';
// }

// .md-signal_wifi_1_bar:after {
//   content: '\e5da';
// }

// .md-signal_wifi_1_bar:before {
//   content: '\e5db';
// }

// .md-signal_wifi_1_bar_lock:after {
//   content: '\e5dc';
// }

// .md-signal_wifi_1_bar_lock:before {
//   content: '\e5dd';
// }

// .md-signal_wifi_2_bar:after {
//   content: '\e5de';
// }

// .md-signal_wifi_2_bar:before {
//   content: '\e5df';
// }

// .md-signal_wifi_2_bar_lock:after {
//   content: '\e5e0';
// }

// .md-signal_wifi_2_bar_lock:before {
//   content: '\e5e1';
// }

// .md-signal_wifi_3_bar:after {
//   content: '\e5e2';
// }

// .md-signal_wifi_3_bar:before {
//   content: '\e5e3';
// }

// .md-signal_wifi_3_bar_lock:after {
//   content: '\e5e4';
// }

// .md-signal_wifi_3_bar_lock:before {
//   content: '\e5e5';
// }

// .md-signal_wifi_4_bar:after {
//   content: '\e5e6';
// }

// .md-signal_wifi_4_bar_lock:after {
//   content: '\e5e7';
// }

// .md-signal_wifi_off:after {
//   content: '\e5e8';
// }

// .md-sim_card:after {
//   content: '\e5e9';
// }

// .md-sim_card_alert:after {
//   content: '\e5eb';
// }

// .md-single_bed:after {
//   content: '\e5ed';
// }

// .md-skip_next:after {
//   content: '\e5ef';
// }

// .md-skip_previous:after {
//   content: '\e5f1';
// }

// .md-slideshow:after {
//   content: '\e5f3';
// }

// .md-slow_motion_video:after {
//   content: '\e5f5';
// }

// .md-smartphone:after {
//   content: '\e5f6';
// }

// .md-smoke_free:after {
//   content: '\e5f8';
// }

// .md-smoking_rooms:after {
//   content: '\e5f9';
// }

// .md-sms:after {
//   content: '\e5fb';
// }

// .md-sms_failed:after {
//   content: '\e5fd';
// }

// .md-snooze:after {
//   content: '\e5ff';
// }

// .md-sort:after {
//   content: '\e600';
// }

// .md-sort_by_alpha:after {
//   content: '\e601';
// }

// .md-spa:after {
//   content: '\e602';
// }

// .md-space_bar:after {
//   content: '\e604';
// }

// .md-speaker:after {
//   content: '\e605';
// }

// .md-speaker_group:after {
//   content: '\e607';
// }

// .md-speaker_notes:after {
//   content: '\e609';
// }

// .md-speaker_notes_off:after {
//   content: '\e60b';
// }

// .md-speaker_phone:after {
//   content: '\e60d';
// }

// .md-speed:after {
//   content: '\e60f';
// }

// .md-spellcheck:after {
//   content: '\e610';
// }

// .md-sports:after {
//   content: '\e611';
// }

// .md-sports_baseball:after {
//   content: '\e612';
// }

// .md-sports_basketball:after {
//   content: '\e614';
// }

// .md-sports_cricket:after {
//   content: '\e616';
// }

// .md-sports_esports:after {
//   content: '\e618';
// }

// .md-sports_football:after {
//   content: '\e61a';
// }

// .md-sports_golf:after {
//   content: '\e61c';
// }

// .md-sports_handball:after {
//   content: '\e61e';
// }

// .md-sports_hockey:after {
//   content: '\e61f';
// }

// .md-sports_kabaddi:after {
//   content: '\e620';
// }

// .md-sports_mma:after {
//   content: '\e621';
// }

// .md-sports_motorsports:after {
//   content: '\e623';
// }

// .md-sports_rugby:after {
//   content: '\e625';
// }

// .md-sports_soccer:after {
//   content: '\e627';
// }

// .md-sports_tennis:after {
//   content: '\e629';
// }

// .md-sports_volleyball:after {
//   content: '\e62a';
// }

// .md-square_foot:after {
//   content: '\e62c';
// }

// .md-star:after {
//   content: '\e62e';
// }

// .md-star_border:after {
//   content: '\e630';
// }

// .md-star_half:after {
//   content: '\e631';
// }

// .md-star_rate:after {
//   content: '\e632';
// }

// .md-stars:after {
//   content: '\e633';
// }

// .md-stay_current_landscape:after {
//   content: '\e635';
// }

// .md-stay_current_portrait:after {
//   content: '\e637';
// }

// .md-stay_primary_landscape:after {
//   content: '\e639';
// }

// .md-stay_primary_portrait:after {
//   content: '\e63b';
// }

// .md-stop:after {
//   content: '\e63d';
// }

// .md-stop_circle:after {
//   content: '\e63f';
// }

// .md-stop_screen_share:after {
//   content: '\e641';
// }

// .md-storage:after {
//   content: '\e643';
// }

// .md-store:after {
//   content: '\e644';
// }

// .md-store_mall_directory:after {
//   content: '\e646';
// }

// .md-storefront:after {
//   content: '\e648';
// }

// .md-straighten:after {
//   content: '\e64a';
// }

// .md-streetview:after {
//   content: '\e64c';
// }

// .md-strikethrough_s:after {
//   content: '\e64d';
// }

// .md-style:after {
//   content: '\e64e';
// }

// .md-subdirectory_arrow_left:after {
//   content: '\e650';
// }

// .md-subdirectory_arrow_right:after {
//   content: '\e651';
// }

// .md-subject:after {
//   content: '\e652';
// }

// .md-subscriptions:after {
//   content: '\e653';
// }

// .md-subtitles:after {
//   content: '\e655';
// }

// .md-subway:after {
//   content: '\e657';
// }

// .md-supervised_user_circle:after {
//   content: '\e659';
// }

// .md-supervisor_account:after {
//   content: '\e65b';
// }

// .md-surround_sound:after {
//   content: '\e65d';
// }

// .md-swap_calls:after {
//   content: '\e65f';
// }

// .md-swap_horiz:after {
//   content: '\e660';
// }

// .md-swap_horizontal_circle:after {
//   content: '\e661';
// }

// .md-swap_vert:after {
//   content: '\e663';
// }

// .md-swap_vertical_circle:after {
//   content: '\e664';
// }

// .md-switch_camera:after {
//   content: '\e666';
// }

// .md-switch_video:after {
//   content: '\e668';
// }

// .md-sync:after {
//   content: '\e66a';
// }

// .md-sync_alt:after {
//   content: '\e66b';
// }

// .md-sync_disabled:after {
//   content: '\e66c';
// }

// .md-sync_problem:after {
//   content: '\e66d';
// }

// .md-system_update:after {
//   content: '\e66e';
// }

// .md-system_update_alt:after {
//   content: '\e670';
// }

// .md-tab:after {
//   content: '\e671';
// }

// .md-tab_unselected:after {
//   content: '\e672';
// }

// .md-table_chart:after {
//   content: '\e673';
// }

// .md-tablet:after {
//   content: '\e675';
// }

// .md-tablet_android:after {
//   content: '\e677';
// }

// .md-tablet_mac:after {
//   content: '\e679';
// }

// .md-tag_faces:after {
//   content: '\e67b';
// }

// .md-tap_and_play:after {
//   content: '\e67d';
// }

// .md-terrain:after {
//   content: '\e67e';
// }

// .md-text_fields:after {
//   content: '\e680';
// }

// .md-text_format:after {
//   content: '\e681';
// }

// .md-text_rotate_up:after {
//   content: '\e682';
// }

// .md-text_rotate_vertical:after {
//   content: '\e683';
// }

// .md-text_rotation_angledown:after {
//   content: '\e684';
// }

// .md-text_rotation_angleup:after {
//   content: '\e685';
// }

// .md-text_rotation_down:after {
//   content: '\e686';
// }

// .md-text_rotation_none:after {
//   content: '\e687';
// }

// .md-textsms:after {
//   content: '\e688';
// }

// .md-texture:after {
//   content: '\e68a';
// }

// .md-theaters:after {
//   content: '\e68b';
// }

// .md-thumb_down:after {
//   content: '\e68d';
// }

// .md-thumb_down_alt:after {
//   content: '\e68f';
// }

// .md-thumb_up:after {
//   content: '\e691';
// }

// .md-thumb_up_alt:after {
//   content: '\e693';
// }

// .md-thumbs_up_down:after {
//   content: '\e695';
// }

// .md-time_to_leave:after {
//   content: '\e697';
// }

// .md-timelapse:after {
//   content: '\e699';
// }

// .md-timeline:after {
//   content: '\e69b';
// }

// .md-timer:after {
//   content: '\e69c';
// }

// .md-timer_10:after {
//   content: '\e69e';
// }

// .md-timer_3:after {
//   content: '\e69f';
// }

// .md-timer_off:after {
//   content: '\e6a0';
// }

// .md-title:after {
//   content: '\e6a2';
// }

// .md-toc:after {
//   content: '\e6a3';
// }

// .md-today:after {
//   content: '\e6a4';
// }

// .md-toggle_off:after {
//   content: '\e6a6';
// }

// .md-toggle_on:after {
//   content: '\e6a8';
// }

// .md-toll:after {
//   content: '\e6aa';
// }

// .md-tonality:after {
//   content: '\e6ac';
// }

// .md-touch_app:after {
//   content: '\e6ae';
// }

// .md-toys:after {
//   content: '\e6b0';
// }

// .md-track_changes:after {
//   content: '\e6b2';
// }

// .md-traffic:after {
//   content: '\e6b3';
// }

// .md-train:after {
//   content: '\e6b5';
// }

// .md-tram:after {
//   content: '\e6b7';
// }

// .md-transfer_within_a_station:after {
//   content: '\e6b9';
// }

// .md-transform:after {
//   content: '\e6ba';
// }

// .md-transit_enterexit:after {
//   content: '\e6bb';
// }

// .md-translate:after {
//   content: '\e6bc';
// }

// .md-trending_down:after {
//   content: '\e6bd';
// }

// .md-trending_flat:after {
//   content: '\e6be';
// }

// .md-trending_up:after {
//   content: '\e6bf';
// }

// .md-trip_origin:after {
//   content: '\e6c0';
// }

// .md-tune:after {
//   content: '\e6c1';
// }

// .md-turned_in:after {
//   content: '\e6c2';
// }

// .md-turned_in_not:after {
//   content: '\e6c4';
// }

// .md-tv:after {
//   content: '\e6c5';
// }

// .md-tv_off:after {
//   content: '\e6c7';
// }

// .md-unarchive:after {
//   content: '\e6c9';
// }

// .md-undo:after {
//   content: '\e6cb';
// }

// .md-unfold_less:after {
//   content: '\e6cc';
// }

// .md-unfold_more:after {
//   content: '\e6cd';
// }

// .md-unsubscribe:after {
//   content: '\e6ce';
// }

// .md-update:after {
//   content: '\e6d0';
// }

// .md-usb:after {
//   content: '\e6d1';
// }

// .md-verified_user:after {
//   content: '\e6d2';
// }

// .md-vertical_align_bottom:after {
//   content: '\e6d4';
// }

// .md-vertical_align_center:after {
//   content: '\e6d5';
// }

// .md-vertical_align_top:after {
//   content: '\e6d6';
// }

// .md-vertical_split:after {
//   content: '\e6d7';
// }

// .md-vibration:after {
//   content: '\e6d9';
// }

// .md-video_call:after {
//   content: '\e6db';
// }

// .md-video_label:after {
//   content: '\e6dd';
// }

// .md-video_library:after {
//   content: '\e6df';
// }

// .md-videocam:after {
//   content: '\e6e1';
// }

// .md-videocam_off:after {
//   content: '\e6e3';
// }

// .md-videogame_asset:after {
//   content: '\e6e5';
// }

// .md-view_agenda:after {
//   content: '\e6e7';
// }

// .md-view_array:after {
//   content: '\e6e9';
// }

// .md-view_carousel:after {
//   content: '\e6eb';
// }

// .md-view_column:after {
//   content: '\e6ed';
// }

// .md-view_comfy:after {
//   content: '\e6ef';
// }

// .md-view_compact:after {
//   content: '\e6f1';
// }

// .md-view_day:after {
//   content: '\e6f3';
// }

// .md-view_headline:after {
//   content: '\e6f5';
// }

// .md-view_list:after {
//   content: '\e6f6';
// }

// .md-view_module:after {
//   content: '\e6f8';
// }

// .md-view_quilt:after {
//   content: '\e6fa';
// }

// .md-view_stream:after {
//   content: '\e6fc';
// }

// .md-view_week:after {
//   content: '\e6fe';
// }

// .md-vignette:after {
//   content: '\e700';
// }

// .md-visibility:after {
//   content: '\e702';
// }

// .md-visibility_off:after {
//   content: '\e704';
// }

// .md-voice_chat:after {
//   content: '\e706';
// }

// .md-voice_over_off:after {
//   content: '\e708';
// }

// .md-voicemail:after {
//   content: '\e70a';
// }

// .md-volume_down:after {
//   content: '\e70b';
// }

// .md-volume_mute:after {
//   content: '\e70d';
// }

// .md-volume_off:after {
//   content: '\e70f';
// }

// .md-volume_up:after {
//   content: '\e711';
// }

// .md-vpn_key:after {
//   content: '\e713';
// }

// .md-vpn_lock:after {
//   content: '\e715';
// }

// .md-wallpaper:after {
//   content: '\e717';
// }

// .md-warning:after {
//   content: '\e718';
// }

// .md-watch:after {
//   content: '\e71a';
// }

// .md-watch_later:after {
//   content: '\e71c';
// }

// .md-waves:after {
//   content: '\e71e';
// }

// .md-wb_auto:after {
//   content: '\e71f';
// }

// .md-wb_cloudy:after {
//   content: '\e721';
// }

// .md-wb_incandescent:after {
//   content: '\e723';
// }

// .md-wb_iridescent:after {
//   content: '\e725';
// }

// .md-wb_sunny:after {
//   content: '\e727';
// }

// .md-wc:after {
//   content: '\e729';
// }

// .md-web:after {
//   content: '\e72a';
// }

// .md-web_asset:after {
//   content: '\e72c';
// }

// .md-weekend:after {
//   content: '\e72e';
// }

// .md-whatshot:after {
//   content: '\e730';
// }

// .md-where_to_vote:after {
//   content: '\e732';
// }

// .md-widgets:after {
//   content: '\e734';
// }

// .md-wifi:after {
//   content: '\e736';
// }

// .md-wifi_lock:after {
//   content: '\e737';
// }

// .md-wifi_off:after {
//   content: '\e738';
// }

// .md-wifi_tethering:after {
//   content: '\e739';
// }

// .md-work:after {
//   content: '\e73a';
// }

// .md-work_off:after {
//   content: '\e73c';
// }

// .md-work_outline:after {
//   content: '\e73e';
// }

// .md-wrap_text:after {
//   content: '\e73f';
// }

// .md-youtube_searched_for:after {
//   content: '\e740';
// }

// .md-zoom_in:after {
//   content: '\e741';
// }

// .md-zoom_out:after {
//   content: '\e742';
// }

// .md-zoom_out_map:after {
//   content: '\e743';
// }

// .md-add_ic_call:after {
//   content: '\e744';
// }

// .md-library_add_check:after {
//   content: '\e746';
// }

// .md-star_outline:after {
//   content: '\e748';
// }

// .md-two_wheeler:after {
//   content: '\e749';
// }

// .md-5g:after {
//   content: '\e74a';
// }

// .md-ad_units:after {
//   content: '\e74b';
// }

// .md-add_business:after {
//   content: '\e74d';
// }

// .md-add_location_alt:after {
//   content: '\e74f';
// }

// .md-add_road:after {
//   content: '\e751';
// }

// .md-addchart:after {
//   content: '\e752';
// }

// .md-admin_panel_settings:after {
//   content: '\e753';
// }

// .md-agriculture:after {
//   content: '\e755';
// }

// .md-alt_route:after {
//   content: '\e757';
// }

// .md-analytics:after {
//   content: '\e758';
// }

// .md-anchor:after {
//   content: '\e75a';
// }

// .md-api:after {
//   content: '\e75b';
// }

// .md-app_blocking:after {
//   content: '\e75c';
// }

// .md-app_settings_alt:after {
//   content: '\e75e';
// }

// .md-architecture:after {
//   content: '\e760';
// }

// .md-arrow_circle_down:after {
//   content: '\e761';
// }

// .md-arrow_circle_up:after {
//   content: '\e763';
// }

// .md-article:after {
//   content: '\e765';
// }

// .md-attach_email:after {
//   content: '\e767';
// }

// .md-auto_delete:after {
//   content: '\e768';
// }

// .md-baby_changing_station:after {
//   content: '\e76a';
// }

// .md-backpack:after {
//   content: '\e76b';
// }

// .md-backup_table:after {
//   content: '\e76d';
// }

// .md-batch_prediction:after {
//   content: '\e76f';
// }

// .md-bedtime:after {
//   content: '\e771';
// }

// .md-bike_scooter:after {
//   content: '\e773';
// }

// .md-biotech:after {
//   content: '\e774';
// }

// .md-browser_not_supported:after {
//   content: '\e776';
// }

// .md-build_circle:after {
//   content: '\e777';
// }

// .md-calculate:after {
//   content: '\e779';
// }

// .md-campaign:after {
//   content: '\e77b';
// }

// .md-charging_station:after {
//   content: '\e77d';
// }

// .md-checkroom:after {
//   content: '\e77f';
// }

// .md-cleaning_services:after {
//   content: '\e780';
// }

// .md-close_fullscreen:after {
//   content: '\e782';
// }

// .md-comment_bank:after {
//   content: '\e783';
// }

// .md-construction:after {
//   content: '\e785';
// }

// .md-corporate_fare:after {
//   content: '\e786';
// }

// .md-design_services:after {
//   content: '\e788';
// }

// .md-directions_off:after {
//   content: '\e78a';
// }

// .md-do_not_step:after {
//   content: '\e78b';
// }

// .md-do_not_touch:after {
//   content: '\e78d';
// }

// .md-domain_verification:after {
//   content: '\e78f';
// }

// .md-dry:after {
//   content: '\e791';
// }

// .md-dynamic_form:after {
//   content: '\e793';
// }

// .md-edit_road:after {
//   content: '\e795';
// }

// .md-electric_bike:after {
//   content: '\e797';
// }

// .md-electric_car:after {
//   content: '\e798';
// }

// .md-electric_moped:after {
//   content: '\e79a';
// }

// .md-electric_scooter:after {
//   content: '\e79c';
// }

// .md-electrical_services:after {
//   content: '\e79d';
// }

// .md-elevator:after {
//   content: '\e79e';
// }

// .md-engineering:after {
//   content: '\e7a0';
// }

// .md-escalator:after {
//   content: '\e7a2';
// }

// .md-escalator_warning:after {
//   content: '\e7a4';
// }

// .md-fact_check:after {
//   content: '\e7a5';
// }

// .md-family_restroom:after {
//   content: '\e7a7';
// }

// .md-filter_alt:after {
//   content: '\e7a8';
// }

// .md-flaky:after {
//   content: '\e7aa';
// }

// .md-forward_to_inbox:after {
//   content: '\e7ab';
// }

// .md-grading:after {
//   content: '\e7ad';
// }

// .md-handyman:after {
//   content: '\e7ae';
// }

// .md-hearing_disabled:after {
//   content: '\e7b0';
// }

// .md-help_center:after {
//   content: '\e7b1';
// }

// .md-highlight_alt:after {
//   content: '\e7b3';
// }

// .md-history_edu:after {
//   content: '\e7b4';
// }

// .md-history_toggle_off:after {
//   content: '\e7b6';
// }

// .md-home_repair_service:after {
//   content: '\e7b7';
// }

// .md-horizontal_rule:after {
//   content: '\e7b9';
// }

// .md-hourglass_bottom:after {
//   content: '\e7ba';
// }

// .md-hourglass_disabled:after {
//   content: '\e7bc';
// }

// .md-hourglass_top:after {
//   content: '\e7bd';
// }

// .md-hvac:after {
//   content: '\e7bf';
// }

// .md-image_not_supported:after {
//   content: '\e7c1';
// }

// .md-insights:after {
//   content: '\e7c3';
// }

// .md-integration_instructions:after {
//   content: '\e7c4';
// }

// .md-legend_toggle:after {
//   content: '\e7c6';
// }

// .md-login:after {
//   content: '\e7c7';
// }

// .md-maps_ugc:after {
//   content: '\e7c8';
// }

// .md-mark_chat_read:after {
//   content: '\e7ca';
// }

// .md-mark_chat_unread:after {
//   content: '\e7cc';
// }

// .md-mark_email_read:after {
//   content: '\e7ce';
// }

// .md-mark_email_unread:after {
//   content: '\e7d0';
// }

// .md-mediation:after {
//   content: '\e7d2';
// }

// .md-medical_services:after {
//   content: '\e7d3';
// }

// .md-military_tech:after {
//   content: '\e7d5';
// }

// .md-miscellaneous_services:after {
//   content: '\e7d7';
// }

// .md-model_training:after {
//   content: '\e7d8';
// }

// .md-moped:after {
//   content: '\e7d9';
// }

// .md-more_time:after {
//   content: '\e7db';
// }

// .md-multiple_stop:after {
//   content: '\e7dc';
// }

// .md-nat:after {
//   content: '\e7dd';
// }

// .md-next_plan:after {
//   content: '\e7df';
// }

// .md-no_cell:after {
//   content: '\e7e1';
// }

// .md-no_drinks:after {
//   content: '\e7e3';
// }

// .md-no_flash:after {
//   content: '\e7e5';
// }

// .md-no_food:after {
//   content: '\e7e7';
// }

// .md-no_photography:after {
//   content: '\e7e9';
// }

// .md-no_stroller:after {
//   content: '\e7eb';
// }

// .md-not_accessible:after {
//   content: '\e7ed';
// }

// .md-not_started:after {
//   content: '\e7ee';
// }

// .md-online_prediction:after {
//   content: '\e7f0';
// }

// .md-open_in_full:after {
//   content: '\e7f1';
// }

// .md-outlet:after {
//   content: '\e7f2';
// }

// .md-payments:after {
//   content: '\e7f4';
// }

// .md-pedal_bike:after {
//   content: '\e7f6';
// }

// .md-pending:after {
//   content: '\e7f7';
// }

// .md-pending_actions:after {
//   content: '\e7f9';
// }

// .md-person_add_alt_1:after {
//   content: '\e7fb';
// }

// .md-person_remove:after {
//   content: '\e7fd';
// }

// .md-person_remove_alt_1:after {
//   content: '\e7ff';
// }

// .md-person_search:after {
//   content: '\e801';
// }

// .md-pest_control:after {
//   content: '\e803';
// }

// .md-pest_control_rodent:after {
//   content: '\e805';
// }

// .md-plagiarism:after {
//   content: '\e807';
// }

// .md-plumbing:after {
//   content: '\e809';
// }

// .md-point_of_sale:after {
//   content: '\e80a';
// }

// .md-preview:after {
//   content: '\e80c';
// }

// .md-privacy_tip:after {
//   content: '\e80e';
// }

// .md-psychology:after {
//   content: '\e810';
// }

// .md-public_off:after {
//   content: '\e812';
// }

// .md-push_pin:after {
//   content: '\e814';
// }

// .md-qr_code:after {
//   content: '\e816';
// }

// .md-quickreply:after {
//   content: '\e818';
// }

// .md-read_more:after {
//   content: '\e81a';
// }

// .md-receipt_long:after {
//   content: '\e81b';
// }

// .md-request_quote:after {
//   content: '\e81d';
// }

// .md-room_preferences:after {
//   content: '\e81f';
// }

// .md-rule:after {
//   content: '\e821';
// }

// .md-rule_folder:after {
//   content: '\e822';
// }

// .md-run_circle:after {
//   content: '\e824';
// }

// .md-science:after {
//   content: '\e826';
// }

// .md-search_off:after {
//   content: '\e828';
// }

// .md-self_improvement:after {
//   content: '\e829';
// }

// .md-sensor_door:after {
//   content: '\e82a';
// }

// .md-sensor_window:after {
//   content: '\e82c';
// }

// .md-shopping_bag:after {
//   content: '\e82e';
// }

// .md-smart_button:after {
//   content: '\e830';
// }

// .md-snippet_folder:after {
//   content: '\e831';
// }

// .md-soap:after {
//   content: '\e833';
// }

// .md-source:after {
//   content: '\e835';
// }

// .md-stairs:after {
//   content: '\e837';
// }

// .md-stroller:after {
//   content: '\e839';
// }

// .md-subscript:after {
//   content: '\e83b';
// }

// .md-subtitles_off:after {
//   content: '\e83c';
// }

// .md-superscript:after {
//   content: '\e83e';
// }

// .md-support:after {
//   content: '\e83f';
// }

// .md-support_agent:after {
//   content: '\e841';
// }

// .md-switch_left:after {
//   content: '\e842';
// }

// .md-switch_right:after {
//   content: '\e844';
// }

// .md-table_rows:after {
//   content: '\e846';
// }

// .md-table_view:after {
//   content: '\e848';
// }

// .md-text_snippet:after {
//   content: '\e84a';
// }

// .md-topic:after {
//   content: '\e84c';
// }

// .md-tour:after {
//   content: '\e84e';
// }

// .md-tty:after {
//   content: '\e850';
// }

// .md-umbrella:after {
//   content: '\e852';
// }

// .md-upgrade:after {
//   content: '\e854';
// }

// .md-verified:after {
//   content: '\e855';
// }

// .md-video_settings:after {
//   content: '\e857';
// }

// .md-view_sidebar:after {
//   content: '\e858';
// }

// .md-wash:after {
//   content: '\e85a';
// }

// .md-wheelchair_pickup:after {
//   content: '\e85c';
// }

// .md-wifi_calling:after {
//   content: '\e85d';
// }

// .md-wifi_protected_setup:after {
//   content: '\e85f';
// }

// .md-wrong_location:after {
//   content: '\e860';
// }

// .md-wysiwyg:after {
//   content: '\e861';
// }

// .md-bento:after {
//   content: '\e864';
// }

// .md-carpenter:after {
//   content: '\e866';
// }

// .md-closed_caption_disabled:after {
//   content: '\e868';
// }

// .md-countertops:after {
//   content: '\e86a';
// }

// .md-east:after {
//   content: '\e86c';
// }

// .md-fence:after {
//   content: '\e86d';
// }

// .md-fire_extinguisher:after {
//   content: '\e86f';
// }

// .md-food_bank:after {
//   content: '\e870';
// }

// .md-foundation:after {
//   content: '\e872';
// }

// .md-grass:after {
//   content: '\e874';
// }

// .md-house_siding:after {
//   content: '\e875';
// }

// .md-leaderboard:after {
//   content: '\e877';
// }

// .md-microwave:after {
//   content: '\e879';
// }

// .md-near_me_disabled:after {
//   content: '\e87b';
// }

// .md-night_shelter:after {
//   content: '\e87d';
// }

// .md-no_meals:after {
//   content: '\e87f';
// }

// .md-no_transfer:after {
//   content: '\e880';
// }

// .md-north:after {
//   content: '\e882';
// }

// .md-north_east:after {
//   content: '\e883';
// }

// .md-north_west:after {
//   content: '\e884';
// }

// .md-qr_code_scanner:after {
//   content: '\e885';
// }

// .md-rice_bowl:after {
//   content: '\e886';
// }

// .md-roofing:after {
//   content: '\e888';
// }

// .md-set_meal:after {
//   content: '\e88a';
// }

// .md-south:after {
//   content: '\e88c';
// }

// .md-south_east:after {
//   content: '\e88d';
// }

// .md-south_west:after {
//   content: '\e88e';
// }

// .md-sports_bar:after {
//   content: '\e88f';
// }

// .md-sticky_note_2:after {
//   content: '\e891';
// }

// .md-tapas:after {
//   content: '\e893';
// }

// .md-water_damage:after {
//   content: '\e895';
// }

// .md-west:after {
//   content: '\e897';
// }

// .md-wine_bar:after {
//   content: '\e898';
// }

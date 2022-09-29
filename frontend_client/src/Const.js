const REMOTE_HOST = window.location.protocol + "//" + window.location.hostname + ":8000/";
const GET_REMOTE_HOST = function (ext) { return REMOTE_HOST + ext };

export default GET_REMOTE_HOST;
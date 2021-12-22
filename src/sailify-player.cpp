#include "sailify-player.h"

#include <QLoggingCategory>
#include <QDateTime>
#include <QDateTime>

namespace Sailify {

namespace {
Q_LOGGING_CATEGORY(logger, "sailify.player");
}

static inline QString toQString(SailifyStringView ffi_view) {
    return QString::fromUtf8(ffi_view.ptr, ffi_view.len);
}

static inline SailifyStringView toFfi(const QByteArray& utf8) {
    return {
        .ptr = utf8.data(),
        .len = static_cast<size_t>(utf8.size())
    };
}

SailifyPlayer::SailifyPlayer() {
    auto* callback = new SailifyPlayerCallback(this);
    auto ffiCallback = callback->getFfiCallback();
    m_player = ::sailify_player_new(&ffiCallback);

    m_positionTimer.setInterval(1000);

    connect(
        callback, &SailifyPlayerCallback::stopped,
        this, &SailifyPlayer::onStopped);
    connect(
        callback, &SailifyPlayerCallback::changed,
        this, &SailifyPlayer::onChanged);
    connect(
        callback, &SailifyPlayerCallback::loading,
        this, &SailifyPlayer::onLoading);
    connect(
        callback, &SailifyPlayerCallback::playing,
        this, &SailifyPlayer::onPlaying);
    connect(
        callback, &SailifyPlayerCallback::paused,
        this, &SailifyPlayer::onPaused);
    connect(
        callback, &SailifyPlayerCallback::unavailable,
        this, &SailifyPlayer::onUnavailable);
    connect(
        callback, &SailifyPlayerCallback::volumeChanged,
        this, &SailifyPlayer::onVolumeChanged);
    connect(
        callback, &SailifyPlayerCallback::connecting,
        this, &SailifyPlayer::onConnecting);
    connect(
        callback, &SailifyPlayerCallback::connected,
        this, &SailifyPlayer::onConnected);
    connect(
        callback, &SailifyPlayerCallback::error,
        this, &SailifyPlayer::onError);
    connect(
        callback, &SailifyPlayerCallback::shutdown,
        this, &SailifyPlayer::onShutdown);
    connect(
        callback, &SailifyPlayerCallback::startReconnect,
        this, &SailifyPlayer::onStartReconnect);
    connect(
        callback, &SailifyPlayerCallback::tokenChanged,
        this, &SailifyPlayer::onTokenChanged);
}

SailifyPlayer::~SailifyPlayer() {
    sailify_player_delete(m_player);
    m_player = nullptr;
}

QString SailifyPlayer::username() const {
    return toQString(sailify_player_get_username(m_player));
}

void SailifyPlayer::setUsername(const QString& value) {
    QByteArray utf8 = value.toUtf8();
    sailify_player_set_username(m_player, toFfi(utf8));
}

QString SailifyPlayer::password() const {
    return toQString(sailify_player_get_password(m_player));
}

void SailifyPlayer::setPassword(const QString& value) {
    QByteArray utf8 = value.toUtf8();
    sailify_player_set_password(m_player, toFfi(utf8));
}

bool SailifyPlayer::isActive() const {
    return sailify_player_is_active(m_player);
}

QString SailifyPlayer::errorString() const {
    return m_errorString;
}

SailifyPlayer::ErrorKind SailifyPlayer::errorKind() const {
    return m_errorKind;
}

SailifyPlayer::MediaStatus SailifyPlayer::mediaStatus() const {
    return m_mediaStatus;
}

SailifyPlayer::ConnectionStatus SailifyPlayer::connectionStatus() const {
    return m_connectionStatus;
}

QString SailifyPlayer::trackUri() const {
    return m_trackId;
}

SailifyPlayer::PlaybackState SailifyPlayer::playbackState() const {
    return m_playbackState;
}

qint32 SailifyPlayer::position() const {
    return m_positionMs;
}

qint32 SailifyPlayer::duration() const {
    return m_durationMs;
}

QString SailifyPlayer::accessToken() const {
    return m_accessToken;
}

qint64 SailifyPlayer::accessTokenExpiresAt() const {
    return m_accessTokenExpiresAt;
}

QString SailifyPlayer::deviceId() const {
    return toQString(sailify_player_get_device_id(m_player));
}

QString SailifyPlayer::deviceName() const {
    return toQString(sailify_player_get_device_name(m_player));
}

void SailifyPlayer::refreshAccessToken() {
    qCInfo(logger) << "Requested new access token";
    sailify_player_refresh_access_token(m_player);
}

void SailifyPlayer::start() {
    qCInfo(logger) << "Requested start";
    sailify_player_start(m_player);
}

void SailifyPlayer::stop() {
    qCInfo(logger) << "Requested stop";
    sailify_player_stop(m_player);
}

void SailifyPlayer::logout() {
    qCInfo(logger) << "Requested logout";
    sailify_player_logout(m_player);
}

void SailifyPlayer::play() {
    qCInfo(logger) << "Requested play";
    sailify_player_play(m_player);
}

void SailifyPlayer::pause() {
    qCInfo(logger) << "Requested pause";
    sailify_player_pause(m_player);
}

void SailifyPlayer::next() {
    qCInfo(logger) << "Requested next";
    sailify_player_next(m_player);
}

void SailifyPlayer::previous() {
    qCInfo(logger) << "Requested previous";
    sailify_player_previous(m_player);
}

void SailifyPlayer::updatePosition() {

}

void SailifyPlayer::setPlayerStatus(
        const QString& trackId, qint32 positionMs, qint32 durationMs, MediaStatus mediaStatus,
        PlaybackState playbackState) {
    bool changedTrackId = m_trackId != trackId;
    bool changedPositionMs = m_positionMs != positionMs;
    bool changedDurationMs = m_durationMs != durationMs;
    bool changedMediaStatus = m_mediaStatus != mediaStatus;
    bool changedPlaybackState = m_playbackState != playbackState;

    m_trackId = trackId;
    m_positionMs = positionMs;
    m_durationMs = durationMs;
    m_mediaStatus = mediaStatus;
    m_playbackState = playbackState;

    if (m_playbackState == Playing && m_positionMs >= 0) {
        m_positionTimestamp = QDateTime::currentMSecsSinceEpoch();
        m_positionTimer.start();
    } else {
        m_positionTimestamp = -1;
        m_positionTimer.stop();
    }

    if (changedMediaStatus) {
        emit mediaStatusChanged();
    }
    if (changedPlaybackState) {
        emit playbackStateChanged();
    }
    if (changedTrackId) {
        emit trackUriChanged();
    }
    if (changedPositionMs) {
        emit positionChanged();
    }
    if (changedDurationMs) {
        emit durationChanged();
    }
}

void SailifyPlayer::onStopped(quint64 playRequestId, const QString& trackId) {
    qCInfo(logger) << "Stopped:" << trackId << " request:" << playRequestId;

    setPlayerStatus(trackId, -1, -1, NoMedia, Stopped);
}

void SailifyPlayer::onChanged(const QString& newTrackId) {
    qCInfo(logger) << "Track changed:" << newTrackId;

    if (newTrackId != m_trackId) {
        m_trackId = newTrackId;
        emit trackUriChanged();
    }
}

void SailifyPlayer::onLoading(quint64 playRequestId, const QString& trackId, quint32 positionMs) {
    qCInfo(logger) << "Playing:" << trackId << " request:" << playRequestId;

    setPlayerStatus(trackId, positionMs, -1, Loading, Stopped);
}

void SailifyPlayer::onPlaying(quint64 playRequestId, const QString& trackId, quint32 positionMs, quint32 durationMs) {
    qCInfo(logger) << "Playing:" << trackId << " request:" << playRequestId;

    setPlayerStatus(trackId, positionMs, durationMs, Loaded, Playing);
}

void SailifyPlayer::onPaused(quint64 playRequestId, const QString& trackId, quint32 positionMs, quint32 durationMs) {
    qCInfo(logger) << "Paused:" << trackId << " request:" << playRequestId;

    setPlayerStatus(trackId, positionMs, durationMs, Loaded, Paused);
}

void SailifyPlayer::onUnavailable(quint64 playRequestId, const QString& trackId) {
    qCInfo(logger) << "Track unavailable:" << trackId << " request:" << playRequestId;

    setPlayerStatus(trackId, -1, -1, InvalidMedia, Stopped);
}

void SailifyPlayer::onVolumeChanged(quint16 value) {
    qCDebug(logger) << "Volume changed:" << value;
    if (value != m_volume) {
        m_volume = value;
        emit onVolumeChanged(value);
    }
}

void SailifyPlayer::setConnectionStatus(ConnectionStatus value) {
    if (m_connectionStatus == value) {
        return;
    }

    m_connectionStatus = value;
}

void SailifyPlayer::onConnecting() {
    qCInfo(logger) << "Connecting";
    setConnectionStatus(Connecting);
}

void SailifyPlayer::onConnected() {
    qCInfo(logger) << "Connected";
    setConnectionStatus(Connected);
}

void SailifyPlayer::onError(SailifyErrorKind kind, const QString& message) {
    switch (kind) {
        case SailifyErrorKind::MissingCredentials: return setError(MissingCredentials, message);
        case SailifyErrorKind::IllegalConfig: return setError(IllegalConfig, message);
        case SailifyErrorKind::Io: return setError(IoError, message);
        case SailifyErrorKind::Connection: return setError(ConnectionError, message);
        case SailifyErrorKind::Panic:
            setConnectionStatus(Crashed);
            return setError(Panic, message);
        case SailifyErrorKind::Token:
            qCCritical(logger) << "Access token refresh error:" << message;
            return emit accessTokenRefreshFailed(message);
    }
}

void SailifyPlayer::setError(ErrorKind kind, const QString& message) {
    qCCritical(logger) << "Player error:" << message;
    m_errorString = message;
    m_errorKind = kind;
    emit errorOccurred(kind, message);
}

void SailifyPlayer::onShutdown() {
    qCInfo(logger) << "Player shutdown";
    setConnectionStatus(Disconnected);
}

void SailifyPlayer::onStartReconnect() {
    qCInfo(logger) << "Player reconnects";
    setConnectionStatus(Connecting);
}

void SailifyPlayer::onTokenChanged(const QString& accessToken, quint32 expiresIn) {
    qCInfo(logger) << "Access token changed - expiresIn:" << expiresIn;

    m_accessToken = accessToken;
    m_accessTokenExpiresAt = QDateTime::currentMSecsSinceEpoch() + expiresIn;
    emit onTokenChanged(accessToken, expiresIn);
}

::SailifyCallback SailifyPlayerCallback::getFfiCallback() const {
    SailifyCallback callback = {
        .user_data = parent(),
        .stopped = SailifyPlayerCallback::onStopped,
        .changed = SailifyPlayerCallback::onChanged,
        .loading = SailifyPlayerCallback::onLoading,
        .playing = SailifyPlayerCallback::onPlaying,
        .paused = SailifyPlayerCallback::onPaused,
        .unavailable = SailifyPlayerCallback::onUnavailable,
        .volume_changed = SailifyPlayerCallback::onVolumeChanged,
        .connecting = SailifyPlayerCallback::onConnecting,
        .connected = SailifyPlayerCallback::onConnected,
        .error = SailifyPlayerCallback::onError,
        .shutdown = SailifyPlayerCallback::onShutdown,
        .start_reconnect = SailifyPlayerCallback::onStartReconnect,
        .token_changed = SailifyPlayerCallback::onTokenChanged,
    };
    return callback;
}

void SailifyPlayerCallback::onStopped(void *user_data, uint64_t play_request_id, SailifyStringView track_id) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->stopped(play_request_id, toQString(track_id));
}

void SailifyPlayerCallback::onChanged(void *user_data, SailifyStringView new_track_id) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->changed(toQString(new_track_id));
}

void SailifyPlayerCallback::onLoading(void *user_data, uint64_t play_request_id, SailifyStringView track_id, uint32_t position_ms) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->loading(play_request_id, toQString(track_id), position_ms);
}

void SailifyPlayerCallback::onPlaying(void *user_data, uint64_t play_request_id, SailifyStringView track_id, uint32_t position_ms, uint32_t duration_ms) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->playing(play_request_id, toQString(track_id), position_ms, duration_ms);
}

void SailifyPlayerCallback::onPaused(void *user_data, uint64_t play_request_id, SailifyStringView track_id, uint32_t position_ms, uint32_t duration_ms) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->paused(play_request_id, toQString(track_id), position_ms, duration_ms);
}

void SailifyPlayerCallback::onUnavailable(void *user_data, uint64_t play_request_id, SailifyStringView track_id) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->unavailable(play_request_id, toQString(track_id));
}

void SailifyPlayerCallback::onVolumeChanged(void *user_data, uint16_t value) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->volumeChanged(value);
}

void SailifyPlayerCallback::onConnecting(void *user_data) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->connecting();
}

void SailifyPlayerCallback::onConnected(void *user_data) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->connected();
}

void SailifyPlayerCallback::onError(void *user_data, SailifyErrorKind kind, SailifyStringView message) {
    qCCritical(logger) << "onError" << message;
    emit static_cast<SailifyPlayerCallback*>(user_data)->error(kind, toQString(message));
}

void SailifyPlayerCallback::onShutdown(void *user_data) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->shutdown();
}

void SailifyPlayerCallback::onStartReconnect(void *user_data) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->startReconnect();
}

void SailifyPlayerCallback::onTokenChanged(void *user_data, SailifyStringView access_token, uint32_t expires_in) {
    emit static_cast<SailifyPlayerCallback*>(user_data)->tokenChanged(toQString(access_token), expires_in);
}

void SailifyPlayerCallback::onDestroy(void *user_data) {
    delete static_cast<SailifyPlayerCallback*>(user_data);
}

}

#pragma once

#include <QObject>
#include <QString>
#include <QString>
#include <QTimer>
#include <QElapsedTimer>

#include <sailifyplayer.h>

namespace Sailify {

class SailifyPlayer : public QObject {
    Q_OBJECT
    Q_PROPERTY(QString username READ username WRITE setUsername)
    Q_PROPERTY(QString password READ password WRITE setPassword)
    Q_PROPERTY(bool active READ isActive NOTIFY activeChanged)
    Q_PROPERTY(QString errorString READ errorString NOTIFY errorOccurred)
    Q_PROPERTY(ErrorKind errorKind READ errorKind NOTIFY errorOccurred)
    Q_PROPERTY(MediaStatus mediaStatus READ mediaStatus NOTIFY mediaStatusChanged)
    Q_PROPERTY(ConnectionStatus connectionStatus READ connectionStatus NOTIFY connectionStatusChanged)
    Q_PROPERTY(QString trackUri READ trackUri NOTIFY trackUriChanged)
    Q_PROPERTY(PlaybackState playbackState READ playbackState NOTIFY playbackStateChanged)
    Q_PROPERTY(quint32 position READ position NOTIFY positionChanged)
    Q_PROPERTY(quint32 duration READ duration NOTIFY durationChanged)
    Q_PROPERTY(QString accessToken READ accessToken NOTIFY accessTokenChanged)
    Q_PROPERTY(qlonglong accessTokenExpiresAt READ accessTokenExpiresAt)
    Q_PROPERTY(QString deviceId READ deviceId CONSTANT)
    Q_PROPERTY(QString deviceName READ deviceName CONSTANT)
public:
    enum MediaStatus {
        NoMedia = 0,
        Loading = 1,
        Loaded = 2,
        Buffering = 3,
        Stalled = 4,
        Buffered = 5,
        EndOfMedia = 6,
        InvalidMedia = 7,
        UnknownStatus = 8,
    };
    Q_ENUM(MediaStatus)

    enum ConnectionStatus {
        Disconnected = 0,
        Connecting = 1,
        Connected = 2,
    };
    Q_ENUM(ConnectionStatus)

    enum PlaybackState {
        Playing,
        Paused,
        Stopped,
    };
    Q_ENUM(PlaybackState)

    enum ErrorKind {
        NoError,
        MissingCredentials,
        IllegalConfig,
        IoError,
        ConnectionError,
        Panic,
    };
    Q_ENUM(ErrorKind)

    SailifyPlayer();
    ~SailifyPlayer();

    QString username() const;
    void setUsername(const QString& value);

    QString password() const;
    void setPassword(const QString& value);

    bool isActive() const;

    QString errorString() const;
    ErrorKind errorKind() const;

    MediaStatus mediaStatus() const;
    ConnectionStatus connectionStatus() const;
    PlaybackState playbackState() const;
    QString trackUri() const;
    qint32 position() const;
    qint32 duration() const;
    QString accessToken() const;
    qint64 accessTokenExpiresAt() const;
    QString deviceId() const;
    QString deviceName() const;

public slots:
    void refreshAccessToken();
    void start();
    void stop();
    void logout();
    void play();
    void pause();
    void next();
    void previous();
    void updatePosition();

signals:
    void activeChanged(bool active);
    void errorOccurred(ErrorKind kind, const QString& message);
    void mediaStatusChanged(MediaStatus mediaStatus);
    void connectionStatusChanged(ConnectionStatus connectionStatus);
    void trackUriChanged(const QString& trackUri);
    void playbackStateChanged(PlaybackState playbackState);
    void positionChanged(qint32 position);
    void durationChanged(qint32 duration);
    void accessTokenChanged(const QString& accessToken);
    void accessTokenRefreshFailed(const QString& message);

private:
    ::SailifyPlayer* m_player = nullptr;

    QString m_accessToken;
    qint64 m_accessTokenExpiresAt = -1;

    QString m_errorString;
    ErrorKind m_errorKind = NoError;

    MediaStatus m_mediaStatus = NoMedia;
    ConnectionStatus m_connectionStatus = Disconnected;
    PlaybackState m_playbackState = Stopped;

    QString m_trackId;

    QTimer m_positionTimer;
    QElapsedTimer m_positionElapsedTimer;
    qint32 m_positionMs = 0;
    qint32 m_durationMs = 0;

    quint16 m_volume = 0;

    void onStopped(quint64 playRequestId, const QString& trackId);
    void onChanged(const QString& newTrackId);
    void onLoading(quint64 playRequestId, const QString& trackId, quint32 positionMs);
    void onPlaying(quint64 playRequestId, const QString& trackId, quint32 positionMs, quint32 durationMs);
    void onPaused(quint64 playRequestId, const QString& trackId, quint32 positionMs, quint32 durationMs);
    void onUnavailable(quint64 playRequestId, const QString& trackId);
    void onVolumeChanged(quint16 value);
    void onConnecting();
    void onConnected();
    void onError(SailifyErrorKind kind, const QString& message);
    void onShutdown();
    void onStartReconnect();
    void onTokenChanged(const QString& accessToken, quint32 expiresIn);

    void setError(ErrorKind kind, const QString& message);
    void setPlayerStatus(
        const QString& trackId, qint32 positionMs, qint32 durationMs, MediaStatus mediaStatus,
        PlaybackState playbackState);
    void setConnectionStatus(ConnectionStatus value);
};

class SailifyPlayerCallback : public QObject {
    Q_OBJECT
public:
    SailifyPlayerCallback() {
        moveToThread(nullptr);
    }

    ::SailifyCallback createFfiCallback();

signals:
    void stopped(quint64 play_request_id, const QString& track_id);
    void changed(const QString& new_track_id);
    void loading(quint64 play_request_id, const QString& track_id, quint32 position_ms);
    void playing(quint64 play_request_id, const QString& track_id, quint32 position_ms, quint32 duration_ms);
    void paused(quint64 play_request_id, const QString& track_id, quint32 position_ms, quint32 duration_ms);
    void unavailable(quint64 play_request_id, const QString& track_id);
    void volumeChanged(quint16 value);
    void connecting();
    void connected();
    void error(SailifyErrorKind kind, const QString& message);
    void shutdown();
    void startReconnect();
    void tokenChanged(const QString& access_token, quint32 expires_in);

    void destroy();

private:
    static void onStopped(void *user_data, uint64_t play_request_id, SailifyStringView track_id);
    static void onChanged(void *user_data, SailifyStringView new_track_id);
    static void onLoading(void *user_data, uint64_t play_request_id, SailifyStringView track_id, uint32_t position_ms);
    static void onPlaying(void *user_data, uint64_t play_request_id, SailifyStringView track_id, uint32_t position_ms, uint32_t duration_ms);
    static void onPaused(void *user_data, uint64_t play_request_id, SailifyStringView track_id, uint32_t position_ms, uint32_t duration_ms);
    static void onUnavailable(void *user_data, uint64_t play_request_id, SailifyStringView track_id);
    static void onVolumeChanged(void *user_data, uint16_t value);
    static void onConnecting(void *user_data);
    static void onConnected(void *user_data);
    static void onError(void *user_data, SailifyErrorKind kind, SailifyStringView message);
    static void onShutdown(void *user_data);
    static void onStartReconnect(void *user_data);
    static void onTokenChanged(void *user_data, SailifyStringView access_token, uint32_t expires_in);

    static void onDestroy(void *data);
};

}

Q_DECLARE_METATYPE(SailifyErrorKind)

#include <memory>

#include <sailfishapp.h>
#include <QDBusConnection>
#include <QLoggingCategory>
#include <QGuiApplication>

#include "sailify-config.h"
#include "sailifyplayer.h"

static Q_LOGGING_CATEGORY(logger, "sailify.app")

using namespace Sailify;

int main(int argc, char *argv[]) {
    std::unique_ptr<QGuiApplication> app(SailfishApp::application(argc, argv));

    // Instance check
    QDBusConnection sessionBus = QDBusConnection::sessionBus();
    if (!sessionBus.registerService(DBUS_SERVICE_NAME)) {
        qCInfo(logger) << "Other instance exists";
        //UI::raise();
        return 0;
    }

    SailifyPlayer* player = sailify_player_new(nullptr);
    sailify_player_delete(player);

    return app->exec();
}
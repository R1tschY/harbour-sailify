#include <memory>

#include <sailfishapp.h>
#include <QDBusConnection>
#include <QLoggingCategory>
#include <QGuiApplication>
#include <QtQuick>

#include "sailify-config.h"
#include "sailify-player.h"

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

    qmlRegisterType<Sailify::SailifyPlayer>("Sailify", 0, 1, "SpotifyPlayer");

    QQuickView* view = SailfishApp::createView();
    view->setSource(SailfishApp::pathToMainQml());
    view->showFullScreen();

    return app->exec();
}
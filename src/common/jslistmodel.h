#ifndef JSONLISTMODEL_H
#define JSONLISTMODEL_H

#include <QAbstractListModel>
#include <QJSValue>

class QJSEngine;

class JsListModel : public QAbstractListModel {
    Q_OBJECT
    Q_PROPERTY(QString values WRITE setValues NOTIFY valuesChanged)
    Q_PROPERTY(QStringList properties READ properties WRITE setProperties NOTIFY propertiesChanged)

public:
    explicit JsListModel(QObject* parent = nullptr);

    // Basic functionality:
    int rowCount(const QModelIndex& parent = QModelIndex()) const override;

    QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override;
    QHash<int, QByteArray> roleNames() const override;

    QJSValue values(QJSEngine* jsengine) const;
    void setValues(const QJSValue& values);

    QStringList properties() const;
    void setProperties(const QStringList& properties);

signals:
    void valuesChanged();
    void propertiesChanged();

private:
    QVector<QJSValue> m_values;
    QStringList m_properties;
};

#endif // JSONLISTMODEL_H

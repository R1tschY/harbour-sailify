#ifndef JSONLISTMODEL_H
#define JSONLISTMODEL_H

#include <QAbstractListModel>
#include <QStringList>
#include <QVector>

class QJsonObject;
class QJSValue;

class JsonListModel : public QAbstractListModel {
    Q_OBJECT
    Q_PROPERTY(QString values READ values WRITE setValues NOTIFY valuesChanged)
    Q_PROPERTY(QStringList properties READ properties WRITE setProperties NOTIFY propertiesChanged)

public:
    explicit JsonListModel(QObject* parent = nullptr);
    virtual ~JsonListModel();

    // Basic functionality:
    int rowCount(const QModelIndex& parent = QModelIndex()) const override;

    QVariant data(const QModelIndex& index, int role = Qt::DisplayRole) const override;
    QHash<int, QByteArray> roleNames() const override;

    QString values() const;
    void setValues(const QString& values);

    QStringList properties() const;
    void setProperties(const QStringList& properties);

    Q_INVOKABLE QVariant get(int index) const;
    Q_INVOKABLE void add(const QString& value);
    Q_INVOKABLE void insert(int index, const QString& value);
    Q_INVOKABLE void extend(const QString& values);
    Q_INVOKABLE void remove(int index);
    Q_INVOKABLE void clear();

    static void registerQmlType();

signals:
    void valuesChanged();
    void propertiesChanged();

private:
    QVector<QJsonObject> m_values;
    QStringList m_properties;
};

#endif // JSONLISTMODEL_H

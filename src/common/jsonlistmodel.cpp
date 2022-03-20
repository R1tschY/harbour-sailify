#include "jsonlistmodel.h"

#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonParseError>
#include <QJSValue>
#include <QLoggingCategory>
#include <QtQml>

static Q_LOGGING_CATEGORY(logger, "Qommons.JsonListModel");

static QJsonDocument parseJson(const QString& json) {
    QJsonParseError error;
    QJsonDocument doc = QJsonDocument::fromJson(json.toUtf8(), &error);
    if (error.error != QJsonParseError::NoError) {
        qCWarning(logger).nospace().noquote()
                << "Invalid json: "
                << error.errorString()
                << ": "
                << json;
    }
    return doc;
}

static QJsonObject toJsonObject(const QJsonDocument& doc) {
    if (!doc.isObject()) {
        qCWarning(logger).nospace().noquote()
                << "Is not a JSON object: "
                << doc;
    }
    return doc.object();

}

static QVector<QJsonObject> toJsonArray(const QJsonArray& array) {
    QVector<QJsonObject> elements;
    elements.reserve(elements.size() + array.size());
    for (const auto& elem : array) {
        if (!elem.isObject()) {
            qCWarning(logger).nospace().noquote()
                    << "Is not a JSON object: "
                    << elem;
        }
        elements.append(elem.toObject());
    }
    return elements;
}

static QVector<QJsonObject> toJsonArray(const QJsonDocument& doc) {
    if (!doc.isArray()) {
        qCWarning(logger).nospace().noquote()
                << "Is not a JSON array: "
                << doc;
    }

    return toJsonArray(doc.array());
}

static QString toJson(const QVector<QJsonObject>& array) {
    auto elements = QJsonArray();
    for (const auto& elem : array) {
        elements.append(QJsonValue(elem));
    }
    return QString::fromUtf8(QJsonDocument(elements).toJson());

}

JsonListModel::JsonListModel(QObject* parent)
    : QAbstractListModel(parent)
{
}

JsonListModel::~JsonListModel() = default;

int JsonListModel::rowCount(const QModelIndex& parent) const
{
    // For list models only the root node (an invalid parent) should return the list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not become a tree model.
    if (parent.isValid())
        return 0;

    return m_values.size();
}

QVariant JsonListModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid())
        return QVariant();

    int row = index.row();
    if (row < 0 || row >= m_values.size())
        return QVariant();
    QJsonObject value = m_values[row];

    int pindex = role - Qt::UserRole;
    if (pindex == m_properties.size()) // modelData
        return value.toVariantMap();

    if (pindex < 0 || pindex > m_properties.size())
        return QVariant();

    return value[m_properties[pindex]].toVariant();
}

QHash<int, QByteArray> JsonListModel::roleNames() const
{
    QHash<int, QByteArray> roles;
    int i = Qt::UserRole;
    for (const QString& prop : m_properties) {
        roles.insert(i++, prop.toUtf8());
    }
    roles.insert(i++ , "modelData");
    return roles;
}

QString JsonListModel::values() const
{
    return toJson(m_values);
}

void JsonListModel::setValues(const QString& values)
{
    beginResetModel();

    QJsonDocument doc = parseJson(values);
    m_values = QVector<QJsonObject>();
    if (!doc.isNull()) {
        m_values = toJsonArray(doc);
    }
    emit valuesChanged();

    endResetModel();
}

QStringList JsonListModel::properties() const
{
    return m_properties;
}

void JsonListModel::setProperties(const QStringList& properties)
{
    if (m_properties != properties) {
        beginResetModel();
        m_properties = properties;
        emit propertiesChanged();
        endResetModel();
    }
}

QVariant JsonListModel::get(int index) const
{
    if (index < 0 || index >= m_values.size())
        return QVariant();
    return m_values[index].toVariantMap();
}

void JsonListModel::add(const QString &value)
{
    beginInsertRows(QModelIndex(), m_values.size(), m_values.size());

    QJsonDocument doc = parseJson(value);
    if (doc.isNull()) {
        m_values.append(QJsonObject());
    } else {
        m_values.append(toJsonObject(doc));
    }
    emit valuesChanged();

    endInsertRows();
}

void JsonListModel::insert(int index, const QString &value)
{
    beginInsertRows(QModelIndex(), index, index);

    QJsonDocument doc = parseJson(value);
    if (doc.isNull()) {
        m_values.insert(index, QJsonObject());
    } else {
        m_values.insert(index, toJsonObject(doc));
    }
    emit valuesChanged();

    endInsertRows();
}

void JsonListModel::extend(const QString &values)
{
    QJsonDocument doc = parseJson(values);
    if (!doc.isNull()) {
        QVector<QJsonObject> newValues = toJsonArray(doc);

        beginInsertRows(
            QModelIndex(), m_values.size(), m_values.size() + newValues.size());
        m_values.append(newValues);
        emit valuesChanged();
        endInsertRows();
    }
}

void JsonListModel::remove(int index)
{
    beginRemoveRows(QModelIndex(), index, index);

    m_values.remove(index);
    emit valuesChanged();

    endRemoveRows();
}

void JsonListModel::clear()
{
    beginRemoveRows(QModelIndex(), 0, m_values.size() - 1);

    m_values.clear();
    emit valuesChanged();

    endRemoveRows();
}

void JsonListModel::registerQmlType()
{
    qmlRegisterType<JsonListModel>(
        "Qommons.Models", 0, 1, "JsonListModel");
}

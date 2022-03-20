#include "jslistmodel.h"

#include <QJSEngine>

JsListModel::JsListModel(QObject* parent)
    : QAbstractListModel(parent)
{
}

int JsListModel::rowCount(const QModelIndex& parent) const
{
    // For list models only the root node (an invalid parent) should return the list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not become a tree model.
    if (parent.isValid())
        return 0;

    return m_values.size();
}

QVariant JsListModel::data(const QModelIndex& index, int role) const
{
    if (!index.isValid())
        return QVariant();

    int row = index.row();
    if (row < 0 || row >= m_values.size())
        return QVariant();
    QJSValue value = m_values[row];

    if (role == Qt::DisplayRole)
        return value.toVariant();

    if (!value.isObject())
        return QVariant();

    int pindex = role - Qt::UserRole;
    if (pindex < 0 || pindex > m_properties.size())
        return QVariant();

    return value.property(m_properties[pindex]).toVariant();
}

QHash<int, QByteArray> JsListModel::roleNames() const
{
    QHash<int, QByteArray> roles;
    int i = Qt::UserRole;
    for (const QString& prop : m_properties) {
        roles.insert(i++, prop.toUtf8());
    }
    return roles;
}

QJSValue JsListModel::values(QJSEngine* jsengine) const
{
    QJSValue result = jsengine->newArray(m_values.size());
    for (int i = 0; i < m_values.size(); ++i) {
        result.setProperty(i, m_values[i]);
    }
    return result;
}

void JsListModel::setValues(const QJSValue& values)
{
    if (!values.isArray())
        return;

    beginResetModel();

    m_values.clear();
    const int length = values.property("length").toInt();
    for (int i = 0; i < length; ++i) {
        m_values.append(values.property(i));
    }

    emit valuesChanged();
    endResetModel();
}

QStringList JsListModel::properties() const
{
    return m_properties;
}

void JsListModel::setProperties(const QStringList& properties)
{
    beginResetModel();
    m_properties = properties;
    emit propertiesChanged();
    endResetModel();
}

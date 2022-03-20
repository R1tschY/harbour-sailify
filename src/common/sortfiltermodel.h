/*
 * Copyright 2019 Richard Liebscher <richard.liebscher@gmail.com>.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef FILTERVALUEPROXYMODEL_H
#define FILTERVALUEPROXYMODEL_H

#include <QString>
#include <QVariant>
#include <QSortFilterProxyModel>
#include <QQmlParserStatus>

namespace Det {

class SortFilterModel : public QSortFilterProxyModel, public QQmlParserStatus
{
    Q_OBJECT
    Q_INTERFACES(QQmlParserStatus)

    Q_PROPERTY(QVariant filterValue
               READ filterValue
               WRITE setFilterValue)

    Q_PROPERTY(QString filterRole
               READ filterRole
               WRITE setFilterRole)

    Q_PROPERTY(bool invertFilter
               READ invertFilter
               WRITE setInvertFilter)

    Q_PROPERTY(QString sortRole
               READ sortRole
               WRITE setSortRole)

    Q_PROPERTY(bool sortAscending
               READ sortAscending
               WRITE setSortAscending)

public:
    SortFilterModel();

    QVariant filterValue() const;
    void setFilterValue(const QVariant &filterValue);

    QString filterRole() const;
    void setFilterRole(const QString& filterRole);


    QString sortRole() const;
    void setSortRole(const QString& sortRole);

    bool sortAscending() const;
    void setSortAscending(bool sortAscending);

    bool invertFilter() const;
    void setInvertFilter(bool invertFilter);

protected:
    bool filterAcceptsRow(
            int source_row, const QModelIndex& source_parent) const override;

private:
    QVariant m_filterValue;
    QString m_filterRole;
    QString m_sortRole;
    bool m_sortAscending = true;
    bool m_invertFilter = false;
    bool m_complete = true;

    void componentComplete() override;
    void classBegin() override;

    void invalidateFilter();
    void onRoleNamesChanged();
};

} // namespace SailfishConnect

#endif // FILTERVALUEPROXYMODEL_H

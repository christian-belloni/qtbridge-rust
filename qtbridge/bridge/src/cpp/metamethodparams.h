// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef METAMETHODPARAMS_H
#define METAMETHODPARAMS_H
#include <QMetaMethod>
#include <QVariant>
#include <vector>
#include "rustconv.h"

#include "rust/cxx.h" // TODO: move specializations for Rust types out of this file


class MetaMethodOutgoingParams
{
public:
    MetaMethodOutgoingParams() = default;

    template <typename ...Args>
    MetaMethodOutgoingParams(Args&&... args)
    {
        m_varData.reserve(sizeof...(args));
        (push(std::forward<Args>(args)), ...);
    }

    void push(QVariant value)
    {
        m_varData.push_back(std::move(value));
    }

    std::vector<void*> getDataPtrs(const QMetaMethod& method);

private:
    std::vector<QVariant> m_varData;
};


namespace rust
{

template <>
struct IsRelocatable<MetaMethodOutgoingParams> : ::std::true_type {};

namespace bridge
{

MetaMethodOutgoingParams MetaMethodOutgoingParams_New();

} // namespace bridge

} // namespace rust

#endif // METAMETHODPARAMS_H

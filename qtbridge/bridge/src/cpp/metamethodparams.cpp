// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#include "metamethodparams.h"

MetaMethodIncomingParams::MetaMethodIncomingParams(const QMetaMethod& method, void** paramData)
    : m_method(method)
    , m_paramData(paramData)
{
}

std::vector<void*> MetaMethodOutgoingParams::getDataPtrs(const QMetaMethod& method)
{
    if (method.parameterCount() != static_cast<int>(m_varData.size()))
        throw std::logic_error("Parameter count mismatch");

    for (size_t i = 0; i < m_varData.size(); ++i) {
        auto& v = m_varData[i];
        if (!v.isValid())
            throw std::runtime_error("Parameter is not set");

        const QMetaType paramType(method.parameterType(static_cast<int>(i)));
        if (!v.convert(paramType))
            throw std::logic_error("Parameter value can not be converted");
    }

    std::vector<void*> result;
    result.reserve(m_varData.size() + 1);
    result.push_back(nullptr); // Reserved for return value

    for (auto& v : m_varData)
        result.push_back(v.data());

    return result;
}

namespace rust::bridge
{

MetaMethodOutgoingParams MetaMethodOutgoingParams_New()
{
    return MetaMethodOutgoingParams();
}

} // namespace rust::bridge

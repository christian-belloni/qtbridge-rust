// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef METAMETHODPARAMS_H
#define METAMETHODPARAMS_H
#include <QMetaMethod>
#include <QVariant>
#include <vector>
#include <stdexcept>
#include "rustconv.h"

#include "rust/cxx.h" // TODO: move specializations for Rust types out of this file

class MetaMethodIncomingParams
{
public:
    MetaMethodIncomingParams(const QMetaMethod& method, void** paramData);

    template <typename T>
    auto getT(size_t paramNum) const
    {
        const int iParamNum = static_cast<int>(paramNum);
        if (iParamNum >= m_method.parameterCount())
            throw std::logic_error("Wrong argument number");

        const QMetaType srcType(m_method.parameterType(iParamNum));
        if (!srcType.isValid())
            throw std::logic_error("Invalid parameter type");

        using ResultT = std::decay_t<T>;
        const QMetaType dstType = QMetaType::fromType<ResultT>();
        if (!dstType.isValid())
            throw std::logic_error("Invalid result type");

        if (!QMetaType::canConvert(srcType, dstType))
            throw std::logic_error("Type can not be converted");

        ResultT result = {}; // TODO: check if it must be done on uninitialized placeholder rather than on default constructed instance
        if (!QMetaType::convert(srcType, const_cast<void*>(m_paramData[iParamNum+1]), dstType, &result))
            throw std::runtime_error("Failed to perform type conversion");

        return result;
    }

    // Seems that its not possible to use the same template function for different rust overloads from Cxx
    // when template type is in return type but not in a argument.
    bool     get_bool    (size_t paramNum) const { return getT<bool>(paramNum); }
    int64_t  get_int64_t (size_t paramNum) const { return getT<int64_t>(paramNum); }
    uint64_t get_uint64_t(size_t paramNum) const { return getT<uint64_t>(paramNum); }
    int32_t  get_int32_t (size_t paramNum) const { return getT<int32_t>(paramNum); }
    uint32_t get_uint32_t(size_t paramNum) const { return getT<uint32_t>(paramNum); }
    int16_t  get_int16_t (size_t paramNum) const { return getT<int16_t>(paramNum); }
    uint16_t get_uint16_t(size_t paramNum) const { return getT<uint16_t>(paramNum); }
    int8_t   get_int8_t  (size_t paramNum) const { return getT<int8_t>(paramNum); }
    uint8_t  get_uint8_t (size_t paramNum) const { return getT<uint8_t>(paramNum); }
    float    get_float   (size_t paramNum) const { return getT<float>(paramNum); }
    double   get_double  (size_t paramNum) const { return getT<double>(paramNum); }

    rust::String getString(size_t paramNum) const
    {
        return QStringToRustString(getT<QString>(paramNum));
    }

    rust::Vec<rust::String> getStringList(size_t paramNum) const
    {
        return QStringListToRustStringList(getT<QStringList>(paramNum));
    }

private:
    QMetaMethod m_method;
    void* const * const m_paramData;
};

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

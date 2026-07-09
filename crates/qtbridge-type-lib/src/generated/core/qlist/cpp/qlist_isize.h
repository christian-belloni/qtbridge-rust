// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

#ifndef _QLIST_ISIZE_RUST_BRIDGE_H_
#define _QLIST_ISIZE_RUST_BRIDGE_H_

#include <QList>
#include <cstdint>
#include "rust/cxx.h"

using QList_isize = ::QList<ptrdiff_t>;

namespace rust::bridge::qlist_isize {

void QList_Drop(QList_isize &v);
QList_isize QList_Default();
QList_isize QList_Clone(const QList_isize &src);
bool QList_Eq(const QList_isize &lhs, const QList_isize &rhs);

void inlineCppFn_append(QList_isize &self, ptrdiff_t value);

size_t inlineCppFn_capacity(QList_isize const &self);

void inlineCppFn_clear(QList_isize &self);

bool inlineCppFn_contains(QList_isize const &self, ptrdiff_t const &value);

void inlineCppFn_push_back(QList_isize &self, ptrdiff_t value);

void inlineCppFn_remove(QList_isize &self, ptrdiff_t i, ptrdiff_t n);

void inlineCppFn_reserve(QList_isize &self, size_t size);

ptrdiff_t inlineCppFn_size(QList_isize const &self);

ptrdiff_t const &inlineCppFn_first(QList_isize const &self);

ptrdiff_t const &inlineCppFn_last(QList_isize const &self);

rust::Vec<ptrdiff_t>
inlineCppFn_TraitImpl_From_ref_QList_isize_for_Vec_isize_from(QList_isize const &src);

ptrdiff_t const *
inlineCppFn_TraitImpl_std_ops_Index_usize_for_QList_isize_index(QList_isize const &self,
                                                                size_t index);

bool inlineCppFn_TraitImpl_PartialEq_array_of_isize_N_for_QList_isize_eq(
        QList_isize const &self, rust::Slice<ptrdiff_t const> rhs);

} // namespace rust::bridge::qlist_isize

namespace rust::bridge::qlist_isize::detail {
struct IsRelocatableDedupDummyTag
{
};
} // namespace rust::bridge::qlist_isize::detail

namespace rust {

template <>
struct IsRelocatable<typename ::std::conditional<
        (::std::is_same<ptrdiff_t, int32_t>::value || ::std::is_same<ptrdiff_t, int64_t>::value),
        ::rust::bridge::qlist_isize::detail::IsRelocatableDedupDummyTag, ::QList_isize>::type>
    : ::std::true_type
{
};

} // namespace rust

#endif // _QLIST_ISIZE_RUST_BRIDGE_H_

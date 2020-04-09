#include "method_handle.hpp"

std::string method_handle_kind_name(method_handle_kind of)
{
  switch (of)
  {
  case method_handle_kind::GetField:
    return "GetField";
  case method_handle_kind::GetStatic:
    return "GetStatic";
  case method_handle_kind::PutField:
    return "PutField";
  case method_handle_kind::PutStatic:
    return "PutStatic";
  case method_handle_kind::InvokeVirtual:
    return "InvokeVirtual";
  case method_handle_kind::InvokeStatic:
    return "InvokeStatic";
  case method_handle_kind::InvokeSpecial:
    return "InvokeSpecial";
  case method_handle_kind::NewInvokeSpecial:
    return "NewInvokeSpecial";
  case method_handle_kind::InvokeInterface:
    return "InvokeInterface";
  }
  return "";
}

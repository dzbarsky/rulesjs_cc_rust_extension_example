#include <cstdint>
#include <utility>
#include <v8.h>

extern "C" void v8_create_external_array_buffer(
  uint8_t* data,
  size_t len,
  size_t capacity,
  v8::Local<v8::ArrayBuffer> *out,
  void (*release_bytes)(void* data, size_t length, void* capacity)
) {
  printf("hello from C\n");
  v8::Isolate *isolate = v8::Isolate::GetCurrent();
  auto bs = v8::ArrayBuffer::NewBackingStore(
    data,
    len,
    release_bytes,
    (void*)capacity);
  *out = v8::ArrayBuffer::New(isolate, std::move(bs));
}

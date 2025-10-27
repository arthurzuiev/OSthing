/* HelloWorldApp.c */

// Libraries
#include <Uefi.h>
#include <Library/UefiBootServicesTableLib.h>
#include <Library/UefiApplicationEntryPoint.h>
#include <Library/UefiLib.h>

// Entry point
EFI_STATUS
EFIAPI

// Main function
HelloWorldAppEntry (
  IN EFI_HANDLE ImageHandle,
  IN EFI_SYSTEM_TABLE *SystemTable
  )
{
  if (SystemTable == NULL) {
    Print(L"Evil Scary Error has eaten UEFI....\n"); // Prints when an error occurs
    return EFI_LOAD_ERROR;
  }

  Print(L"Hello, UEFI Speaking!\n"); // Hope 100%. Means it works
  return EFI_SUCCESS;
}

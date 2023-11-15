import * as React from "react";

export const useDisclosure = (
  initial = false
): {
  isOpen: boolean;
  open: () => void;
  close: () => void;
  toggle: () => void;
} => {
  const [isOpen, setIsOpen] = React.useState(initial);

  const open = React.useCallback(() => {
    setIsOpen(true);
  }, []);
  const close = React.useCallback(() => {
    setIsOpen(false);
  }, []);
  const toggle = React.useCallback(() => {
    setIsOpen((state) => !state);
  }, []);

  return { isOpen, open, close, toggle };
};

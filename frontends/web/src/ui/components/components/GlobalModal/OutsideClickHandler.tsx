import React from "react";
import { useRef, useEffect } from "react";

export const OutsideClickHandler = ({
  children,
  onOutsideClick,
}: {
  children: React.ReactNode;
  onOutsideClick: () => void;
}) => {
  const wrapperRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (wrapperRef.current && !wrapperRef.current.contains(event.target as Node)) {
        onOutsideClick();
      }
    }

    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [wrapperRef, onOutsideClick]);

  return <div ref={wrapperRef}>{children}</div>;
};

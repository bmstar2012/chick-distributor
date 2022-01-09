import React from "react";
import {
  Link,
} from "react-router-dom";
import {
  Box,
  Button,
  Divider,
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemText,
  Stack,
} from "@mui/material";
import HomeIcon from '@mui/icons-material/Home';
import MenuIcon from '@mui/icons-material/Menu';

export const Header = ({ narrow } : { narrow : boolean }) => {
  const navs = [
    {
      href: `/gumdrop/`,
      innerNarrow: "About",
      inner: <HomeIcon />,
    },
    {
      href: `/gumdrop/create`,
      inner: "Create",
    },
    {
      href: `/gumdrop/claim`,
      inner: "Claim",
    },
    {
      href: `/gumdrop/close`,
      inner: "Close",
    },
  ];

  const [drawerOpen, setDrawerOpen] = React.useState(false);

  const toggleDrawer = (open: boolean | ((prevState: boolean) => boolean)) => (event: { type: string; key: string; }) => {
    if (event.type === 'keydown' && (event.key === 'Tab' || event.key === 'Shift')) {
      return;
    }

    setDrawerOpen(open);
  };

  return (
    <Box
      sx={{
        height: "52px",
        display: "flex",
        bgcolor: "action.disabledBackground",
        overflow: "auto",
      }}
    >
      <Box sx={{flexGrow: 1, minWidth: "36px"}}/>
    </Box>
  );
};

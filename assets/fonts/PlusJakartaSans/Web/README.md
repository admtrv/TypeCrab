# Installing Webfonts
Follow these simple Steps.

## 1.
Put `plus-jakarta-sans/` Folder into a Folder called `fonts/`.

## 2.
Put `plus-jakarta-sans.css` into your `css/` Folder.

## 3. (Optional)
You may adapt the `url('path')` in `plus-jakarta-sans.css` depends on your Website Filesystem.

## 4.
Import `plus-jakarta-sans.css` at the top of you main Stylesheet.

```
@import url('plus-jakarta-sans.css');
```

## 5.
You are now ready to use the following Rules in your CSS to specify each Font Style:
```
font-family: PlusJakartaSans-ExtraLight;
font-family: PlusJakartaSans-ExtraLightItalic;
font-family: PlusJakartaSans-Light;
font-family: PlusJakartaSans-LightItalic;
font-family: PlusJakartaSans-Regular;
font-family: PlusJakartaSans-Italic;
font-family: PlusJakartaSans-Medium;
font-family: PlusJakartaSans-MediumItalic;
font-family: PlusJakartaSans-SemiBold;
font-family: PlusJakartaSans-SemiBoldItalic;
font-family: PlusJakartaSans-Bold;
font-family: PlusJakartaSans-BoldItalic;
font-family: PlusJakartaSans-ExtraBold;
font-family: PlusJakartaSans-ExtraBoldItalic;
font-family: PlusJakartaSans-Variable;
font-family: PlusJakartaSans-VariableItalic;

```
## 6. (Optional)
Use `font-variation-settings` rule to controll axes of variable fonts:
wght 400.0

Available axes:
'wght' (range from 200.0 to 800.0


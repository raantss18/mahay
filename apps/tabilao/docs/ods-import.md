# Importing OpenDocument spreadsheets

Open an `.ods` file with File → Open, drag it onto Tabilao, or pass it to the CLI. Tabilao
imports worksheet data into a new workbook and shows a warning describing the conversion limits.

The import reads sheets, numbers, text, booleans, supported dates/times, repeated cells/rows and
merged cells. Formula cells become their **last saved results**, not live formulas. These results
may be out of date if the source application did not recalculate before saving. Unsupported or
missing values retain available display text and produce a warning.

This is data import, not full ODS compatibility. Source formatting, formulas, charts, images,
names and other workbook features are not preserved. Tabilao does not write ODS files.

Save opens Save As and suggests a new `.xlsx` file. The source ODS file is not used as the
workbook's save destination; requests to save to `.ods` return an error.

```sh
tabilao-cli info source.ods
tabilao-cli convert source.ods imported.xlsx
```

CLI import warnings go to stderr, leaving structured output on stdout available for scripts.

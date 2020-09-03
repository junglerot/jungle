package org.enso.launcher.cli

/**
  * Gathers settings set by the global CLI options.
  *
  * @param autoConfirm if this flag is set, the program should not ask the user
  *                    any questions but proceed with the default values, that
  *                    must be explained in the help text for each command
  * @param hideProgress if this flag is set, progress bars should not be
  *                     printed
  * @param useJSON specifies if output should be in JSON format, if it is
  *                supported (currently only the version command supports JSON)
  */
case class GlobalCLIOptions(
  autoConfirm: Boolean,
  hideProgress: Boolean,
  useJSON: Boolean
)

{ lib }:

{
  mkConfig = { config, devMode, amberPath, configPath, sourcePath }:
    if devMode
    then config.lib.file.mkOutOfStoreSymlink (toString "${amberPath}/${configPath}")
    else sourcePath;

  mkConfigFile = { config, devMode, amberPath, configPath, sourcePath }:
    if devMode
    then config.lib.file.mkOutOfStoreSymlink (toString "${amberPath}/${configPath}")
    else sourcePath;
}

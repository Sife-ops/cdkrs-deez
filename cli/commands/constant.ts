import config from "../config";

const { APP_ID, GUILD_ID, BOT_TOKEN } = config;
const { GLSST_GLOBAL } = process.env;

export const url = GLSST_GLOBAL
  ? `https://discord.com/api/v10/applications/${APP_ID}/commands`
  : `https://discord.com/api/v10/applications/${APP_ID}/guilds/${GUILD_ID}/commands`;

export const headers = {
  Authorization: `Bot ${BOT_TOKEN}`,
  "Content-Type": "application/json",
};

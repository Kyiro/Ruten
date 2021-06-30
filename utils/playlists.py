from datetime import datetime
import requests

data = "[/Script/FortniteGame.FortGameInstance]\n!FrontEndPlaylistData=ClearArray\n"
req = requests.get("https://fortnite-api.com/v1/playlists").json()
now = datetime.now().strftime('%Y-%m-d_%H %M %S.ini')
file = open(now, "w")

for i in req["data"]:
    category = 0
    display = 1
    if "[PH]" in i['name']:
        category = 1
        display = 0
    elif "Apollo" in i['id']:
        category = 1
        display = 2
    elif i['gameplayTags'] and "Athena.Creative.LTM" in i['gameplayTags']:
        category = 2
    data += f"+FrontEndPlaylistData=(PlaylistName={i['id']}, PlaylistAccess=(bEnabled=true, CategoryIndex={category} AdvertiseType=New, DisplayPriority={display}))\n"

# print(data)
file.write(data)
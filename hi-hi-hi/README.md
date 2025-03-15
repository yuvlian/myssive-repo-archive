# hi-hi-hi

honkai impact 3 --> hi3 --> hi hi hi

currently only supports bh3 cn prod 8.1.0 cuz im lazy

for the database:
- `cargo install sqlx-cli`

- `setx DATABASE_URL sqlite:sqlite.db` // restart vscode after this

- `sqlx db create`

- `sqlx migrate run`

fiddler script:

```
import System;
import System.Windows.Forms;
import Fiddler;
import System.Text.RegularExpressions;

class Handlers
{
    static function OnBeforeRequest(oS: Session) {
        if (
            (oS.host.EndsWith(".hoyoverse.com") || 
            oS.host.EndsWith(".mihoyo.com") || 
            oS.host.EndsWith(".yuanshen.com") || 
            oS.host.EndsWith(".bhsr.com") || 
            oS.host.EndsWith(".starrails.com") || 
            oS.host.EndsWith(".juequling.com") || 
            oS.host.EndsWith(".zenlesszonezero.com") || 
            oS.host.EndsWith(".bh3.com") || 
            oS.host.EndsWith(".honkaiimpact3.com") || 
            oS.host.EndsWith(".mob.com") || 
            oS.host.EndsWith(".hyg.com"))
            && !(
                oS.fullUrl.ToLower().Contains("https://bundle-qcloud.bh3.com/asset_bundle/android01/1.0") || 
                oS.fullUrl.ToLower().Contains("https://bundle.bh3.com/asset_bundle/android01/1.0") || 
                oS.fullUrl.ToLower().Contains("bundle-qcloud.bh3.com/tmp/Original") || 
                oS.fullUrl.ToLower().Contains("bundle.bh3.com/tmp/Original")
            )
        ) {
            oS.oRequest.headers.UriScheme = "http";
            oS.host = "127.0.0.1";
            oS.port = 21000;
        }
    }
};
```
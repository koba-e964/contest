/*
 * Global variable: ll mod (< 2^62)
 */
ll add(ll x, ll y) {
    ll ans = x + y;
    if (ans >= mod) ans -= mod;
    return ans;
}

ll mul(ll x, ll y) {
    ll ans = 0;
    ll cur = x;
    while (y > 0) {
        if (y % 2 == 1) {
            ans = add(ans, cur);
        }
        cur = add(cur, cur);
        y /= 2;
    }
    return ans;
}

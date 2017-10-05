#include <iostream>
#include <vector>

using namespace std;

#define rep(i, n) for(int i = 0; i < int(n); ++i)

const long mod = 1e9 + 7;
const int N = 123456;
vector<int> edges[N];

long k;
long dp[N];

long invmod(long x) {
    long e = mod - 2;
    long sum = 1;
    long cur = x;
    while (e > 0) {
        if (e % 2 == 1) {
            sum = sum * cur % mod;
        }
        cur = cur * cur % mod;
        e /= 2;
    }
    return sum;
}

long perm(long k, long x) {
    long ans = 1;
    for (long i = 0; i < x; ++i) {
        ans = ans * (k - i) % mod;
    }
    return ans;
}

long dfs(int v, int par) {
    int nc = edges[v].size();
    if (par != -1) {
        nc -= 1;
    }
    long tmp = perm(par == -1 ? (k - 1) : (k - 2), nc);
    rep(i, edges[v].size()) {
        int w = edges[v][i];
        if (w == par) { continue; }
        long sub = dfs(w, v);
        tmp = tmp * sub % mod;
    }
    if (par == -1) {
        tmp = tmp * k % mod;
    }
    return tmp;
}

int main() {
    int n;
    cin >> n >> k;
    rep(i, n - 1) {
        int a, b;
        cin >> a >> b;
        a--, b--;
        edges[a].push_back(b);
        edges[b].push_back(a);
    }
    cout << dfs(0, -1) << endl;
}

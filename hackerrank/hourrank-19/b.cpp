#include <bits/stdc++.h>

using namespace std;

#define rep(i, n) for (int i = 0; i < int(n); ++i)

int main(){
    int n;
    cin >> n;
    vector<int> s(n);
    int tot = 0;
    for(int s_i = 0; s_i < n; s_i++){
        cin >> s[s_i];
        tot ^= s[s_i];
    }
    vector<int> acc(n + 1, 0);
    rep(i, n) {
        acc[i + 1] = acc[i] ^ s[i];
    }
    map<int, int> freq;
    long long cnt = 0;
    rep(i, n) {
        int v = acc[i];
        // if (freq.count(v) == 0) { freq[v] = 0; }
        freq[v] += 1;
        cnt += freq[acc[i + 1] ^ tot];
    }
    cout << cnt << endl;
    // Print the number of ways Alice can select the range to ensure she wins the game.
    return 0;
}

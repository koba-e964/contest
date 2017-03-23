#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
  
#define rep(i, n) for (int i = 0; i < int(n); ++i)
typedef long long i64;
  
int gcd(int x, int y) {
    return y ? gcd(y, x % y) : x;
}
  
int a[1000];
  
int main() {
    // your code goes here
    int n, m;
    cin >> n >> m;
    rep(i, n) {
        cin >> a[i];
    }
    // Pick nm/gcd(n, m) = lcm(n, m) times.
    i64 tot = 0;
    int g = gcd(n, m);
    rep(i, n / g) {
        vector<int> t;
        rep(j, m) {
            // (i * m + j) % n -th is taken
            t.push_back(a[(i * m + j) % n]);
        }
        sort(t.begin(), t.end());
        tot += t[t.size() - 1] - t[0];
    }
    cout << tot << endl;
    return 0;
}

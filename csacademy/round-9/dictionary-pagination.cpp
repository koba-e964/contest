#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;
#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)


int main() {
    int n, q;
    cin >> n >> q;
    vector<string> s;
    REP(i, 0, n) {
        string t;
        cin >> t;
        s.push_back(t);
    }
    sort(s.begin(), s.end());
    REP(i, 0, q) {
        string u;
        int p;
        cin >> u >> p;
        int idx = lower_bound(s.begin(), s.end(), u) - s.begin();
        cout << idx / p + 1 << endl;
    }
}

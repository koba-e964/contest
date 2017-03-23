#include <cassert>
#include <iostream>
#include <string>
#include <set>
using namespace std;
 
 
//// \sum |S_i| < 26^k
//// 400000 < 26^4 = 676^2, so k = 4 is ok
 
 
const int N = 123456;
string s[N];
#define rep(i, n) for (int i = 0; i < int(n); ++i)
 
 
string dfs(const set<string> &occur, int len, string cur) {
    if (cur.length() == len) {
        if (occur.count(cur) == 0) {
            return cur;
        } else {
            return "";
        }
    }
    rep(i, 26) {
        char c = 'a' + i;
        string res = dfs(occur, len, cur + c);
        if (res.length() > 0) {
            return res;
        }
    }
    return "";
}
 
string search(const set<string> &occur, int len) {
    return dfs(occur, len, "");
}
 
// RUPC D
int main() {
    int n;
    cin >> n;
    rep(i, n) {
        cin >> s[i];
    }
    // Enumerate all substrings of length <= 4
    set<string> occur;
    rep(i, n) {
        int l = s[i].length();
        rep(j, l) {
            rep(k, 4) {
                if (j + k + 1 <= l) {
                    occur.insert(s[i].substr(j, k + 1));
                }
            }
        }
    }
    rep(i, 4) {
        int len = i + 1;
        string result = search(occur, len);
        if (result.length() > 0) {
            cout << result << endl;
            return 0;
        }
    }
    assert (0);
    // your code goes here
    return 0;
}

#include <cassert>
#include <iostream>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef vector<int> VI;

int ask(int v) {
    cout << v % 100 << endl;
    int c;
    cin >> c;
    return c;
}


VI enum_poss(VI bl, VI acc) {
    set<int> poss;
    REP(i, 0, 100) {
        poss.insert(i);
    }
    REP(k, 0, 2) {
        REP(i, -9, 10) {
            poss.erase((bl[k] + i + 100) % 100);
        }
        REP(i, 0, 10) {
            poss.erase((bl[k] + 10 * i) % 100);
        }
    }
    REP(i, 0, acc.size()) {
        poss.erase(acc[i]);
    }
    VI ret(poss.begin(), poss.end());
    return ret;
}

void task(void) {
    VI bl;
    VI wh;
    int pos = 0;
    while (pos < 10) {
        if (bl.size() >= 2) {
            break;
        }
        int tmp = 11 * pos;
        int c = ask(tmp);
        if (c == 1) {
            bl.push_back(tmp);
        } else {
            wh.push_back(tmp);
        }
        pos += 1;
    }
    assert (bl.size() >= 1);
    if (bl.size() == 1) {
        int c = ask(bl[0] + 10);
        assert (c == 1);
        return;
    }
    VI acc;
    REP(i, 0, pos) {
        acc.push_back(11 * i);
    }
    VI poss = enum_poss(bl, acc);
    REP(i, 0, 11 - pos) {
        assert (ask(poss[i]) == 0);
    }
}

int main(void) {
    int t;
    cin >> t;
    while (t--) {
        task();
    }
}

#include <algorithm>
#include <cassert>
#include <cctype>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <deque>
#include <functional>
#include <iomanip>
#include <iostream>
#include <list>
#include <map>
#include <queue>
#include <random>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>
#include <complex>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long int ll;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<int, int> PI;
const ll mod = 1e9 + 7;

typedef complex<double> comp;

int main(void) {
    int t;
    cin >> t;
    while (t--) {
        vector<comp> pt(4);
        comp centre(0, 0);
        REP(i, 0, 4) {
            int x, y;
            cin >> x >> y;
            pt[i] = comp(x, y);
            centre += pt[i];
        }
        centre /= 4.0;
        REP(i, 0, 4) {
            pt[i] -= centre;
        }
        set<pair<double, double> > a;
        set<pair<double, double> > b;
        REP(i, 1, 4) {
            a.insert(make_pair(pt[i].real(), pt[i].imag()));
        }
        comp t = pt[0];
        REP(i, 1, 4) {
            t *= comp(0, 1);
            b.insert(make_pair(t.real(), t.imag()));
        }
        cout << (a == b) << endl;
    }
}

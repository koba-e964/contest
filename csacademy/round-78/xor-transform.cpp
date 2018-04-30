#include <iostream>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define DEBUGP(val) cerr << #val << "=" << val << "\n"


using namespace std;
typedef long long int ll;
typedef vector<int> VI;

struct SplitMix64 {
    unsigned long long nextInt() {
        unsigned long long z = (x += 0x9E3779B97F4A7C15ULL);
        z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9ULL;
        z = (z ^ (z >> 27)) * 0x94D049BB133111EBULL;
        return z ^ (z >> 31);
    }
    
    int random() {
        return 1 + (nextInt() >> 33);
    }
    
    unsigned long long x;
};

int n, m, q;
int n2, m2;
vector<VI> b;
int tbl[1 << 23];

int encp2(int v) {
    int c = 1;
    while (v > c) c *= 2;
    return c;
}

void init(const vector< vector<int> >& A) {
    n2 = encp2(n);
    m2 = encp2(m);
    b = vector<VI>(n2, VI(m2));
    REP(i, 0, n) REP(j, 0, m) b[i][j] = A[i][j];
    for (int k = 1; k < n2; k *= 2) {
        REP(i, 0, n2) {
            if (i & k) {
                REP(j, 0, m2) {
                    b[i][j] ^= b[i ^ k][j];
                }
            }
        }
    }
    for (int k = 1; k < m2; k *= 2) {
        REP(j, 0, m2) {
            if (j & k) {
                REP(i, 0, n2) {
                    b[i][j] ^= b[i][j ^ k];
                }
            }
        }
    }
    REP(i, 0, max(n2, m2)) {
        tbl[i] = b[i & (n2 - 1)][i & (m2 - 1)];
    }
}

int query(int k) {
    return tbl[k & (max(n2, m2) - 1)];
}

#define TEST 0

int main() {
    SplitMix64 rng;
    
    cin >> n >> m >> q >> rng.x;
    
    vector< vector<int> > A(n, vector<int>(m));
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < m; j++) {
#if TEST
            cin >> A[i][j];
#else
            A[i][j] = rng.random();
#endif
        }
    }
    
    init(A);

    unsigned long long hashSol = 0;
    
    for (int i = 1; i <= q; i++) {
#if TEST
        int k;
        cin >> k;
#else
        int k = rng.random();
#endif
        hashSol ^= (unsigned long long)i * query(k);
    }
    
    cout << hashSol << endl;
}

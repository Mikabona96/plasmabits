#include <QGuiApplication>
#include <QTextStream>
#include <KIconLoader>

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    if (argc < 2)
        return 2;

    const QString name = QString::fromLocal8Bit(argv[1]);

    const QString path = KIconLoader::global()->iconPath(
        name,
        KIconLoader::Desktop,
        true
    );

    if (path.isEmpty())
        return 1;

    QTextStream(stdout) << path << Qt::endl;

    return 0;
}
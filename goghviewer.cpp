#include "goghviewer.h"
#include "./ui_goghviewer.h"
#include "imageloader.h"

#include <iostream>
#include <memory>

#include <QFileDialog>
#include <QGraphicsPixmapItem>
#include <QMessageBox>
#include <QColorTransform>
#include <QColor>

#include <librembrandt/proc.h>
#include <rust/cxx.h>

GoghViewer::GoghViewer(QWidget *parent)
    : QMainWindow(parent), ui(new Ui::GoghViewer)
{
    ui->setupUi(this);
    ui->gridWidget->hide();
}

GoghViewer::~GoghViewer()
{
    delete ui;
}

void GoghViewer::on_action_Open_triggered()
{
    GoghViewer::openImage();
}

void GoghViewer::on_btnOpenFile_clicked()
{
    GoghViewer::openImage();
}

void GoghViewer::openImage()
{
    QString selectedFile = QFileDialog::getOpenFileName(this, QString("Select Image File"), NULL, QString("Image Files (*.png *.jpg *.bmp)"));
    GoghViewer::openImage(selectedFile);
}

void GoghViewer::openImage(QString selectedFile)
{
    if (selectedFile.isNull())
    {
        std::cout << "No file selected" << std::endl;
        return;
    }

    std::cout << "Opening image: " << selectedFile.toStdString() << std::endl;

    pm = QPixmap::fromImage(il.loadImage(selectedFile));
    this->setShownImage(pm);

    adjustSize();
}

void GoghViewer::on_actionAbout_triggered()
{
    QMessageBox msgBox(this);
    msgBox.setWindowTitle("About Van Gogh");
    msgBox.setTextFormat(Qt::RichText);
    msgBox.setText(R"(
        🖌️ Van Gogh Image Viewer
        <br/>
        <a href="https://github.com/hdoordt/van-gogh">Git repository</a>
    )");
    msgBox.exec();
}

void GoghViewer::on_pushButton_clicked()
{
    ui->gridWidget->setVisible(!ui->gridWidget->isVisible());
    adjustSize();
}

void GoghViewer::on_btnToolGreyscale_clicked()
{
    if (pm.isNull())
    {
        return;
    }

    QImage image = pm.toImage();
    auto imageWidth = image.width();
    auto imageHeight = image.height();
    auto imageSize = image.sizeInBytes();

    // The buffer of a QImage, like many things in Qt,
    // is reference counted. constBits returns a raw pointer
    // into that shared data. Hold on to the image for
    // as long as imageDataBuf lives for it not to become
    // dangling...
    //  https://doc.qt.io/qt-6/implicit-sharing.html
    const uchar *imageDataBuf = image.constBits();

    // (PART 2) TODO: call your Rust-defined greyscale implementation, and
    // pass it to `setShownImage` using
    // `this->setShownImage(QPixmap::fromImage(new_qimage));`
}

void GoghViewer::on_dialHueRotate_valueChanged(int value)
{
    if (pm.isNull())
    {
        return;
    }

    int degrees = (value * 360) / 100;

    QImage image = pm.toImage();
    auto imageWidth = image.width();
    auto imageHeight = image.height();
    auto imageSize = image.sizeInBytes();

    const uchar *imageDataBuf = image.constBits();

    // (PART 2) TODO: call your Rust-defined hue rotation implementation, and
    // pass it to `setShownImage` using
    // `this->setShownImage(QPixmap::fromImage(new_qimage));`
}

void GoghViewer::on_btnSaveAs_clicked()
{
    if (pm.isNull())
    {
        return;
    }

    QString safeFileName = QFileDialog::getSaveFileName(this, QString("Save Image File"), QString("/home/Pictures/"), tr("Images (*.png *.xpm *.jpg)"));

    QImage image = pm.toImage();

    il.storeImage(image, safeFileName);
}

void GoghViewer::setShownImage(QPixmap pixmap)
{
    scene = std::unique_ptr<QGraphicsScene>(new QGraphicsScene);
    ui->imageViewer->setScene(scene.get());
    QGraphicsPixmapItem *item = new QGraphicsPixmapItem(pixmap);
    scene->addItem(item);
}
